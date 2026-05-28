use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use std::fs;
use std::time::Duration;
use sysmqttd::config::Config;
use sysmqttd::daemon::Daemon;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};
use tokio::time;

#[tokio::test]
async fn test_integration_daemon_discovery_and_publish() {
    let run_docker = std::env::var("RUN_DOCKER_TESTS").unwrap_or_default();
    if run_docker != "true" && run_docker != "1" {
        println!(
            "Skipping docker-based integration test because RUN_DOCKER_TESTS is not 'true' or '1'."
        );
        return;
    }

    // Start a Mosquitto container dynamically using testcontainers
    let mosquitto_container = GenericImage::new("eclipse-mosquitto", "latest")
        .with_wait_for(WaitFor::message_on_stderr("running"))
        .with_exposed_port(1883.tcp())
        .with_cmd(["mosquitto", "-c", "/mosquitto-no-auth.conf"])
        .start()
        .await
        .expect("Failed to start Mosquitto container");

    let mqtt_port = mosquitto_container.get_host_port_ipv4(1883).await.unwrap();

    // Set environment variables for service monitoring and test mock execution
    std::env::set_var("MONITORED_SERVICES", "nginx");
    std::env::set_var("SYSMQTTD_TEST_ENV", "1");

    // 1. Establish connection options to our local docker broker
    let config = Config {
        mqtt_host: "127.0.0.1".to_string(),
        mqtt_port,
        mqtt_user: None,
        mqtt_password: None,
        mqtt_topic_prefix: "homeassistant_test".to_string(),
        net_interface: "wlan0".to_string(),
        gpio_inputs: vec![sysmqttd::gpio_inputs::GpioInputConfig {
            pin: 23,
            name: "Front Door".to_string(),
            device_class: Some("door".to_string()),
        }],
        gpio_outputs: vec![sysmqttd::gpio_outputs::GpioOutputConfig {
            pin: 24,
            name: "Relay 1".to_string(),
        }],
        verbose: true,
    };

    // Create temporary directory for mock GPIO base path
    let temp_gpio_dir = std::env::temp_dir().join("sysmqttd_integration_gpio");
    let _ = fs::remove_dir_all(&temp_gpio_dir);
    fs::create_dir_all(&temp_gpio_dir).unwrap();

    // Create export and unexport mock files
    fs::write(temp_gpio_dir.join("export"), "").unwrap();
    fs::write(temp_gpio_dir.join("unexport"), "").unwrap();

    // Simulate kernel/setup creating the sysfs directory structure for inputs/outputs
    let gpio23 = temp_gpio_dir.join("gpio23");
    fs::create_dir_all(&gpio23).unwrap();
    fs::write(gpio23.join("value"), "0\n").unwrap();
    fs::write(gpio23.join("direction"), "in\n").unwrap();
    fs::write(gpio23.join("edge"), "both\n").unwrap();

    let gpio24 = temp_gpio_dir.join("gpio24");
    fs::create_dir_all(&gpio24).unwrap();
    fs::write(gpio24.join("value"), "0\n").unwrap();
    fs::write(gpio24.join("direction"), "out\n").unwrap();

    let daemon = Daemon::new(config, "integration-tester".to_string())
        .with_gpio_base_path(temp_gpio_dir.clone());

    let daemon_mqtt_client_id = daemon.get_mqtt_options().client_id();

    // 2. Connect verifier client and start collecting messages
    let client_id = "verifier_client".to_string();
    let mut mqttoptions = MqttOptions::new(client_id, "127.0.0.1", mqtt_port);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

    let received_messages = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let received_messages_clone = received_messages.clone();

    // Spawn eventloop in a background task to collect all published messages
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(publish))) => {
                    let mut msgs = received_messages_clone.lock().await;
                    msgs.push(publish);
                }
                Err(e) => {
                    eprintln!("Verifier client poll error: {:?}", e);
                    time::sleep(Duration::from_millis(50)).await;
                }
                _ => {}
            }
        }
    });

    // Subscribe to all homeassistant_test/# topics
    client
        .subscribe("homeassistant_test/#", rumqttc::QoS::AtLeastOnce)
        .await
        .unwrap();

    // Small delay to ensure subscription is active
    time::sleep(Duration::from_millis(500)).await;

    // 3. Setup oneshot shutdown signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

    // 4. Spawn daemon's run loop in a separate task
    let daemon_task = tokio::spawn(async move { daemon.run_with_shutdown(shutdown_rx).await });

    // 5. Run for a short period (7 seconds) to let connection, discovery, and first telemetry states stream
    time::sleep(Duration::from_secs(7)).await;

    // 6. Publish a whitelisted remote command to verify integration handling
    client
        .publish(
            "homeassistant_test/sensor/sysmqttd_integration-tester/command",
            rumqttc::QoS::AtLeastOnce,
            false,
            "reboot",
        )
        .await
        .unwrap();

    // 7. Publish to GPIO output command topic to verify Switch actuation code path
    client
        .publish(
            "homeassistant_test/switch/sysmqttd_integration-tester_pin24/set",
            rumqttc::QoS::AtLeastOnce,
            false,
            "ON",
        )
        .await
        .unwrap();

    // Small delay to allow message processing and execution in daemon's event loop
    time::sleep(Duration::from_secs(2)).await;

    // 8. Send shutdown signal and await daemon task completion
    let _ = shutdown_tx.send(());
    let daemon_result = daemon_task.await.unwrap();
    assert!(
        daemon_result.is_ok(),
        "Daemon execution failed: {:?}",
        daemon_result
    );

    // 9. Process and verify collected messages
    let msgs = received_messages.lock().await;
    println!("Total messages captured: {}", msgs.len());
    assert!(!msgs.is_empty(), "No messages captured by verifier client!");

    // Assert MQTT Client ID conforms to sysmqttd_<hostname>
    assert_eq!(daemon_mqtt_client_id, "sysmqttd_integration-tester");

    let mut availability_online = false;
    let mut core_state_received = false;
    let mut discovery_configs_count = 0;

    for msg in msgs.iter() {
        let topic = &msg.topic;
        let payload_str = std::str::from_utf8(&msg.payload).unwrap_or_default();

        // Verify Client ID is present in topic paths
        assert!(
            topic.contains("sysmqttd_integration-tester"),
            "Topic '{}' does not contain the correct client ID pattern!",
            topic
        );

        // Availability validation
        if topic == "homeassistant_test/sensor/sysmqttd_integration-tester/availability"
            && payload_str == "online"
        {
            availability_online = true;
        }

        // Home Assistant Discovery Payload assertions
        if topic.ends_with("/config") {
            discovery_configs_count += 1;
            let json: serde_json::Value =
                serde_json::from_str(payload_str).expect("Discovery payload is not valid JSON!");
            assert!(json.is_object(), "Discovery payload is not a JSON object!");

            // Check if it is a binary_sensor config
            if topic.contains("/binary_sensor/") {
                if topic.contains("_undervoltage") || topic.contains("_throttled") {
                    let dev_cla = json
                        .get("dev_cla")
                        .expect("System binary sensor missing device class!")
                        .as_str()
                        .expect("dev_cla is not a string!");
                    assert_eq!(dev_cla, "problem", "Device class must be 'problem'!");
                }

                // Verify binary mapping mapping true to "ON" and false to "OFF"
                if topic.contains("_undervoltage") {
                    let val_tpl = json.get("val_tpl").unwrap().as_str().unwrap();
                    assert!(
                        val_tpl.contains("'ON' if value_json.undervoltage_detected else 'OFF'"),
                        "Undervoltage value template mapping is incorrect!"
                    );
                } else if topic.contains("_throttled") {
                    let val_tpl = json.get("val_tpl").unwrap().as_str().unwrap();
                    assert!(
                        val_tpl.contains("'ON' if value_json.throttled else 'OFF'"),
                        "Throttled value template mapping is incorrect!"
                    );
                } else if topic.contains("_pin23") {
                    let payload_on = json.get("payload_on").unwrap().as_str().unwrap();
                    let payload_off = json.get("payload_off").unwrap().as_str().unwrap();
                    assert_eq!(payload_on, "ON");
                    assert_eq!(payload_off, "OFF");
                }
            }
        }

        // State Telemetry Format assertions
        if topic == "homeassistant_test/sensor/sysmqttd_integration-tester/state" {
            core_state_received = true;
            let json: serde_json::Value = serde_json::from_str(payload_str)
                .expect("Telemetry state payload is not valid JSON!");
            let obj = json
                .as_object()
                .expect("Telemetry state is not a flat JSON object!");

            for (key, val) in obj.iter() {
                assert!(
                    val.is_number() || val.is_boolean() || val.is_string(),
                    "Telemetry state key '{}' must contain only numeric, boolean, or string value!",
                    key
                );

                if val.is_number() {
                    let num_str = val.to_string();
                    if let Some(pos) = num_str.find('.') {
                        let decimals = &num_str[pos + 1..];
                        assert!(
                            decimals.len() <= 1,
                            "Float value of key '{}' ({}) has more than 1 decimal place of precision!",
                            key,
                            num_str
                        );
                    }
                }
            }
        }
    }

    assert!(
        availability_online,
        "Expected to receive 'online' availability payload!"
    );
    assert!(
        core_state_received,
        "Expected to receive core telemetry state payload!"
    );
    assert!(
        discovery_configs_count > 0,
        "Expected to receive Home Assistant discovery configs!"
    );

    // Clean up environment variables
    std::env::remove_var("MONITORED_SERVICES");
    std::env::remove_var("SYSMQTTD_TEST_ENV");
    let _ = fs::remove_dir_all(&temp_gpio_dir);
}
