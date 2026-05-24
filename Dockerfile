# Stage 1: Build the Rust binary
FROM rust:slim AS builder

WORKDIR /app
COPY . .
RUN cargo build --release --bin sysmqttd

# Stage 2: Packaging stripped minimal binary in a stable slim environment
FROM debian:stable-slim

# Install system dependencies (ca-certificates for TLS support if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Run as non-root user for security hardening
RUN groupadd -g 10001 sysmqttd && \
    useradd -u 10001 -g sysmqttd -m -s /bin/bash sysmqttd

WORKDIR /var/lib/sysmqttd
COPY --from=builder --chown=sysmqttd:sysmqttd /app/target/release/sysmqttd /usr/bin/sysmqttd

USER sysmqttd

# Default fallback environment variables
ENV MQTT_HOST=mosquitto
ENV MQTT_PORT=1883
ENV MQTT_TOPIC_PREFIX=homeassistant

CMD ["sysmqttd"]
