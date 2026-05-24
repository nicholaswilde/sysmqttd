use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use std::time::Duration;
use sysmqttd::config::Config;
use sysmqttd::daemon::Daemon;
use tokio::time;

#[tokio::test]
async fn test_integration_daemon_discovery_and_publish() {
    if std::env::var("RUN_DOCKER_TESTS").is_err() {
        println!("Skipping docker-based integration test because RUN_DOCKER_TESTS is not set.");
        return;
    }

    // 1. Establish connection options to our local docker broker
    let config = Config {
        mqtt_host: "127.0.0.1".to_string(),
        mqtt_port: 1883,
        mqtt_user: None,
        mqtt_password: None,
        mqtt_topic_prefix: "homeassistant_test".to_string(),
        net_interface: "wlan0".to_string(),
    };

    let daemon = Daemon::new(config, "integration-tester".to_string());

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
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

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
                if let Ok(Event::Incoming(Packet::Publish(publish))) = notification {
                    if publish.topic.contains("sysmqttd_integration-tester_cpu_temp/config") {
                        pub_received = true;
                        break;
                    }
                }
            }
        }
    }

    assert!(
        pub_received,
        "Expected to receive retained Home Assistant Discovery payload on subscription"
    );
}
