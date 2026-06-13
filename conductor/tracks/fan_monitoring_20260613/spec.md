# Specification: Fan Speed Monitoring

## Overview
This track adds the ability to monitor fan speeds on host systems running `sysmqttd` by reading fan speed sensors from the Linux `hwmon` sysfs interface (specifically `/sys/class/hwmon/hwmon*/fan*_input`). The feature can be disabled via a configuration file setting, an environment variable, or a command-line argument.

## Functional Requirements
1. **Fan Speed Auto-discovery:**
   - Identify fan inputs dynamically via `/sys/class/hwmon/hwmon*/fan*_input`.
   - Sort discovered paths/files stably (alphanumerically) to ensure consistent ordering/indexing of fans (e.g., `fan_1`, `fan_2`).
2. **Telemetry Payload Integration:**
   - Retrieve fan speeds in RPM and include them as flattened fields in the telemetry state payload:
     ```json
     {
       "cpu_temperature": 43.5,
       ...
       "fan_1": 1200,
       "fan_2": 1500
     }
     ```
3. **Home Assistant Auto-discovery:**
   - Generate and publish MQTT discovery configurations on startup for each discovered fan as separate Home Assistant sensor entities.
   - Each fan sensor entity will:
     - Be registered under the same parent device.
     - Use the unit of measurement `"RPM"`.
     - Use the state class `"measurement"`.
     - Have a unique ID: `sysmqttd_<hostname>_<fan_id>` (e.g., `sysmqttd_myhost_fan_1`).
4. **Disabling Configuration:**
   - Provide a toggle to disable fan speed monitoring via:
     - CLI Flag: `--no-fan`
     - Config File Key: `no_fan` (boolean, default: `false`)
     - Environment Variable: `SYSMQTTD_NO_FAN` (boolean/string, e.g. `true`)
   - When disabled, no fan sensors are discovered, registered, or included in the telemetry payload.
5. **Mock/Testing Support:**
   - In testing/non-Linux environments where `sysfs_root` is not `/` and no fan files are found, fall back to a mock fan `fan_1` at `1200` RPM.

## Non-Functional Requirements
- Ensure size and memory footprints remain within optimized bounds (RAM < 8MB, Binary Size < 2MB).
- Must build cleanly on `arm-unknown-linux-gnueabihf` target.

## Acceptance Criteria
- `sysmqttd` successfully runs and includes `fan_1` (at 1200 RPM mock speed) in telemetry when run in a test environment with `no_fan = false`.
- Setting `no_fan = true` via CLI, config, or env completely disables fan monitoring and removes fan fields/sensors.
- Auto-discovery configuration is successfully sent for each fan.
- All automated tests pass.
- Binary builds cleanly under `cross build`.

## Out of Scope
- Controlling fan speeds (PWM control). This track is monitoring-only.
