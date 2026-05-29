# Specification: MQTT Secure TLS Encryption via rustls

This specification defines the requirements for adding secure, encrypted TLS connections to the MQTT broker utilizing pure-Rust `rustls` configurations.

## Overview
Enables secure transmission of system telemetry and execution of remote control commands over unsecured public or shared networks.

## Functional Requirements
1. **Configurable TLS Mode:**
   - Add `use_tls` (boolean) parameter to configuration file and CLI flags.
   - Add optional `ca_cert_path` (string) to support custom/self-signed root CA certificates.
2. **Pure-Rust TLS Provider:**
   - Enable `rustls` support inside `rumqttc` (keeping `default-features = false` to prevent linking large OpenSSL dependencies).
3. **Automatic System Certificates Load:**
   - Automatically fall back to loading the host system's native root certificate store when `ca_cert_path` is not provided.
4. **Port Defaulting:**
   - Set the default MQTT broker port to `8883` when `use_tls` is active, maintaining `1883` for non-TLS connections.

## Acceptance Criteria
- Code builds cleanly without linking to system `libssl` or `libcrypto`.
- Secure TLS broker connections work successfully with Home Assistant brokers using both standard and custom CAs.
- Cross-compilation targets remain functional.
