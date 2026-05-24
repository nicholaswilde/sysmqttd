# sysmqttd

> [!WARNING]
> **This is a development version at version v0.1.* and things may change at any time.**

`sysmqttd` is a lightweight, native Rust system monitoring daemon optimized aggressively for low-resource single-board computers (specifically the Raspberry Pi Zero W running DietPi). It collects core system metrics (CPU temperature, RAM utilization, and disk storage utilization) and streams them to a Home Assistant MQTT broker, utilizing Home Assistant MQTT Discovery on startup for zero-configuration integration.

## Features
*   **Negligible footprint:** stripped binary footprint of under **530KB** and only **~4-6MB RAM RSS** usage.
*   **Autodiscovery:** Registers CPU Temperature, RAM Usage, and Disk Utilization under a single parent device in Home Assistant using standard MQTT Discovery.
*   **Asynchronous Polling:** Built on Tokio with an asynchronous, non-blocking 60-second telemetry polling loop.
*   **Multi-arch compilation support:** Task runner ready to build ARMv6, ARMv7, ARM64, and AMD64 targets.

## Quick Start & Dev commands
This project uses [go-task](https://taskfile.dev/) as its task runner.

### List available tasks
```bash
task --list
```

### Run local tests
```bash
task test
```

### Check code coverage
```bash
task coverage
```

### Compile cross-compiled binaries
```bash
task build-all
```

For detailed deployment instructions, refer to [deployment/README.md](deployment/README.md).