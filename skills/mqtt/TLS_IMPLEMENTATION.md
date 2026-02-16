# TLS/SSL Support for MQTT - Implementation Guide

## Status
🚧 **In Progress** - Core infrastructure added, testing required

## Overview
Added TLS/SSL support to the MQTT driver for secure connections to brokers. Supports both server authentication (TLS) and mutual authentication (mTLS).

## Configuration Fields Added

```toml
[[drivers]]
protocol = "mqtt"
id = "mqtt-secure"
target = "mqtts.example.com:8883"

[drivers.params]
use_tls = "true"

# Server authentication (TLS)
ca_cert_path = "/etc/esnode/certs/ca.pem"

# Mutual authentication (mTLS) - Optional
client_cert_path = "/etc/esnode/certs/client.pem"
client_key_path = "/etc/esnode/certs/client-key.pem"
```

## Implementation Details

### Dependencies Added
- `rustls = "0.23"` - Modern TLS library
- `rustls-native-certs = "0.7"` - System certificate loading
- `rustls-pemfile = "2.0"` - PEM file parsing

### TLS Features
1. **Server Authentication**: Verify broker identity using CA certificate
2. **System Certificates**: Fall back to OS certificate store if no CA provided
3. **Mutual TLS (mTLS)**: Optional client certificate authentication
4. **Automatic Port**: Use 8883 for TLS, 1883 for plaintext

## Testing Checklist

### Local mosquitto Testing
```bash
# Generate test certificates
openssl genrsa -out ca.key 2048
openssl req -x509 -new -nodes -key ca.key -sha256 -days 1024 -out ca.pem

openssl genrsa -out server.key 2048
openssl req -new -key server.key -out server.csr
openssl x509 -req -in server.csr -CA ca.pem -CAkey ca.key -CAcreateserial -out server.pem -days 500

# Start Mosquitto with TLS
mosquitto -c mosquitto-tls.conf

# Test ESNODE connection
./esnode-sentinel daemon --config esnode-tls.toml
```

### Production Checklist
- [ ] Test with Let's Encrypt certificates
- [ ] Test with corporate PKI
- [ ] Test certificate expiry handling
- [ ] Test connection failures
- [ ] Benchmark TLS vs plaintext performance
- [ ] Document certificate rotation process

## Security Best Practices

### Certificate Storage
```bash
# Secure permissions
chmod 600 /etc/esnode/certs/client-key.pem
chmod 644 /etc/esnode/certs/client.pem
chmod 644 /etc/esnode/certs/ca.pem
chown esnode:esnode /etc/esnode/certs/*
```

### Configuration Template
```toml
# Production MQTT with TLS
[[drivers]]
protocol = "mqtt"
id = "mqtt-prod"
target = "mqtt.datacenter.internal:8883"

[drivers.params]
use_tls = "true"
ca_cert_path = "/etc/esnode/certs/datacenter-ca.pem"
client_cert_path = "/etc/esnode/certs/esnode-client.pem"
client_key_path = "/etc/esnode/certs/esnode-key.pem"
username = "esnode"
password = "${MQTT_PASSWORD}"  # Or use mTLS instead
topics = "datacenter/#"
qos = "1"
```

## Troubleshooting

### Error: "No private key found"
**Solution**: Ensure private key file is in PEM format and not encrypted.

### Error: "Certificate verify failed"
**Solution**: 
1. Check CA certificate includes full chain
2. Verify broker certificate CN matches hostname
3. Check certificate dates

### Error: "Connection reset"
**Solution**: Verify broker TLS port (usually 8883) and TLS is enabled.

## Performance Impact

### Benchmark Results (Expected)
| Metric | Plaintext | TLS |
|--------|-----------|-----|
| Latency | ~10ms | ~15ms |
| Throughput | 10k msg/s | 8k msg/s |
| CPU | 0.1% | 0.3% |

## Next Steps

1. **Complete Testing**: Run comprehensive tests with real MQTT brokers
2. **Certificate Rotation**: Add automatic certificate reload
3. **OCSP Stapling**: Implement certificate revocation checking
4. **SNI Support**: Add Server Name Indication for multi-tenant brokers
5. **TLS 1.3**: Verify TLS 1.3 support (should work automatically with rustls)

## Alternative: Using Mosquitto Bridge

For simpler deployments, use Mosquitto as a TLS termination proxy:

```conf
# mosquitto-bridge.conf
listener 1883
protocol mqtt

connection esnode-bridge
address mqtts.cloud.example.com:8883
bridge_cafile /etc/mosquitto/certs/ca.pem
bridge_certfile /etc/mosquitto/certs/client.pem
bridge_keyfile /etc/mosquitto/certs/client-key.pem
topic # both 0
```

Then connect ESNODE to local Mosquitto (plaintext):
```toml
[[drivers]]
protocol = "mqtt"
id = "mqtt-bridge"
target = "localhost:1883"
```

---

**Status**: TLS infrastructure complete, testing in progress  
**Priority**: Medium (most MQTT deployments are internal/trusted)  
**Estimated Completion**: Next sprint
