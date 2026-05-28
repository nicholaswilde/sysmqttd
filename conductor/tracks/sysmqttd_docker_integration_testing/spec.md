# Specification: Isolated Container-Driven Integration Testing

This specification defines the design, dev-dependency structure, and assertions for isolated integration testing using `testcontainers`.

## Technical Architecture
Rather than relying on `Taskfile.yml` orchestrating external Docker commands, the integration tests will programmatically spin up a Mosquitto container using `testcontainers` async API.

### 1. Dev-Dependencies
The following dev-dependencies will be added to `Cargo.toml`:
- `testcontainers = { version = "0.23", features = ["tokio"] }` (or latest stable)

### 2. State & Discovery Validation Rules
The integration tests must assert:
1. **MQTT Client ID Format**: The connection client ID must exactly match `sysmqttd_<hostname>`.
2. **Availability Topic & Payload**: The daemon must publish `"online"` to `{prefix}/sensor/sysmqttd_{hostname}/availability` and `"offline"` to the LWT (Last Will and Testament).
3. **Home Assistant Discovery Payload**:
   - Retained config payloads published to `<prefix>/sensor/sysmqttd_<hostname>_<metric>/config`.
   - Payload must be a valid serialized HA Discovery JSON object.
   - For `binary_sensor`, it must be under the `binary_sensor` component, with the correct `problem` device class, mapping true to `"ON"` and false to `"OFF"`.
4. **State Telemetry Format**:
   - Must be a flat JSON object containing only numeric and boolean keys.
   - Precision: Float values rounded to 1 decimal place.

### 3. Execution Control
- Integration tests will be isolated under `#[tokio::test]`.
- The test will dynamically acquire the host port mapped to the Mosquitto container's port `1883` using `container.get_host_port_ipv4(1883)`. This guarantees no port conflicts with existing local services.
