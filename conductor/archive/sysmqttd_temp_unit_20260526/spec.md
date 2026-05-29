# Specification: Temperature Unit Selection (Celsius or Fahrenheit)

This specification defines the requirements for allowing the user to select the temperature unit (Celsius or Fahrenheit) for CPU temperature monitoring, with Fahrenheit as the default.

## Overview
Currently, `sysmqttd` publishes CPU temperature exclusively in Celsius (`°C`). To support users in Fahrenheit-preferring regions, this feature adds a configuration parameter to allow switching the unit. The Celsius-to-Fahrenheit conversion will be performed in Rust before publishing, and the Home Assistant Discovery payload will dynamically adjust its `unit_of_measurement` and unique ID.

## Functional Requirements
1. **Configuration and CLI Overrides**:
   - Add a configuration parameter `temperature_unit` in `sysmqttd.toml` (and other supported config formats).
   - Add a CLI flag `-u` / `--unit` / `--temperature-unit` to override the config file.
   - Support environment variable `SYSMQTTD_TEMPERATURE_UNIT` (and legacy `TEMPERATURE_UNIT`).
   - Valid values: `"C"`, `"F"` (case-insensitive).
2. **Default Behavior**:
   - If not specified, the default unit is **Fahrenheit (`"F"`)**.
3. **Rust-Side Conversion**:
   - If Fahrenheit is selected, perform conversion: F = C * 1.8 + 32.
   - Maintain the standard rounding to 1 decimal place (e.g., `(temp * 10.0).round() / 10.0`).
4. **HA Auto-Discovery**:
   - Update the discovery payload for `cpu_temp`:
     - `unit_of_measurement`: `"°C"` or `"°F"` depending on the active unit.
     - `uniq_id`: Maintain unique registration.
     - `value_template`: Keep `{{ value_json.cpu_temperature }}`.

## Acceptance Criteria
- Run `sysmqttd` with no configuration defaults to Fahrenheit, publishing values converted to `°F` with unit `"°F"` in discovery.
- Specifying `-u C` or `temperature_unit = "C"` publishes Celsius values with unit `"°C"` in discovery.
- Unit tests cover configuration parsing, temperature conversion logic, and discovery payload serialization.
- Cross-compilation for `arm-unknown-linux-gnueabihf` succeeds with no warnings.
