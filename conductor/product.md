# Product: `sysmqttd` System Monitor

## Vision
A highly-efficient, lightweight system monitoring daemon for resource-constrained ARMv6 single-board computers (specifically Raspberry Pi Zero W running DietPi). It seamlessly integrates with Home Assistant via MQTT Discovery and provides real-time system metrics (CPU Temp, RAM usage, Disk usage) with a negligible resource footprint.

## Target Audience
System administrators, makers, and smart home enthusiasts running Home Assistant who want to monitor their low-power single-board computers without incurring high memory or CPU overhead.

## Key Features
1.  **Tiny Footprint:** Native compiled Rust binary optimized for size (< 2MB stripped) and memory (< 8MB RAM).
2.  **Home Assistant Autodiscovery:** Zero-configuration setup in Home Assistant via explicit, retained MQTT discovery payloads published on startup.
3.  **Core System Telemetry:** 60-second updates of CPU Temperature, RAM percentage, and root disk percentage utilization.
4.  **Robust Async Lifecycle:** Asynchronous client loop that handles system startup, intermittent network dropouts, and broker reconnects gracefully.
