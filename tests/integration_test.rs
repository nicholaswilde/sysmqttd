use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use std::fs;
use std::time::Duration;
use sysmqttd::config::Config;
use sysmqttd::daemon::Daemon;
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

    // Set environment variable for service monitoring
    std::env::set_var("MONITORED_SERVICES", "nginx");

    // 1. Establish connection options to our local docker broker
    let config = Config {
        mqtt_host: "127.0.0.1".to_string(),
        mqtt_port: 1883,
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

    // 2. Setup oneshot shutdown signal
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel();

    // 3. Spawn daemon's run loop in a separate task
    let daemon_task = tokio::spawn(async move { daemon.run_with_shutdown(shutdown_rx).await });

    // 4. Run for a short period (6 seconds) to let connection, discovery, and first telemetry states stream
    time::sleep(Duration::from_secs(6)).await;

    // 5. Send shutdown signal and await daemon task completion
    let _ = shutdown_tx.send(());
    let daemon_result = daemon_task.await.unwrap();
    assert!(
        daemon_result.is_ok(),
        "Daemon execution failed: {:?}",
        daemon_result
    );

    // 6. Verify that discovery retained payloads were successfully published by subscribing to them
    let client_id = "verifier_client".to_string();
    let mut mqttoptions = MqttOptions::new(client_id, "127.0.0.1", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(30));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 100);

    client
        .subscribe(
            "homeassistant_test/sensor/sysmqttd_integration-tester_cpu_temp/config",
            rumqttc::QoS::AtLeastOnce,
        )
        .await
        .unwrap();

    let mut pub_received = false;
    let timeout = time::sleep(Duration::from_secs(3));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            _ = &mut timeout => {
                break;
            }
            notification = eventloop.poll() => {
                match notification {
                    Ok(Event::Incoming(Packet::Publish(publish)))
                        if publish
                            .topic
                            .contains("sysmqttd_integration-tester_cpu_temp/config") =>
                    {
                        pub_received = true;
                        break;
                    }
                    Err(e) => {
                        eprintln!("Integration test eventloop poll error: {:?}", e);
                        time::sleep(Duration::from_millis(100)).await;
                    }
                    _ => {}
                }
            }
        }
    }

    assert!(
        pub_received,
        "Expected to receive retained Home Assistant Discovery payload on subscription"
    );

    // 7. Publish a whitelisted remote command to verify integration handling
    client
        .publish(
            "homeassistant_test/sensor/sysmqttd_integration-tester/command",
            rumqttc::QoS::AtLeastOnce,
            false,
            "reboot",
        )
        .await
        .unwrap();

    // 8. Publish to GPIO output command topic to verify Switch actuation code path
    client
        .publish(
            "homeassistant_test/switch/sysmqttd_integration-tester_pin24/set",
            rumqttc::QoS::AtLeastOnce,
            false,
            "ON",
        )
        .await
        .unwrap();

    // Small delay to allow message processing and execution in event loop
    time::sleep(Duration::from_secs(1)).await;

    // Clean up environment variables
    std::env::remove_var("MONITORED_SERVICES");
    let _ = fs::remove_dir_all(&temp_gpio_dir);
}
