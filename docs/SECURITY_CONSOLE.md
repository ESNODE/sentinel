# ESNODE Sentinel | Enterprise Console & Security

The ESNODE Sentinel Cloud Console provides a high-performance, secure, and centralized management interface for global AI infrastructure.

## 🚀 Unified Web Console
Sentinel serves a modern, "Obsidian-Neon" themed web console directly from its high-performance Axum backend.

- **URL**: `http://localhost:9100/` (default)
- **Features**: Real-time telemetry, Power Flux Distribution, Skill Efficiency tracking, and Autonomous Activity Logs.

## 🔐 Enterprise Security Suite

Sentinel is built for production environments requiring strict security and identity management.

### HTTPS / TLS Termination
Sentinel supports native TLS termination. Configure your certificates in `sentinel.toml`:

```toml
[security]
enable_https = true
cert_path = "/path/to/fullchain.pem"
key_path = "/path/to/privkey.pem"
```

### Authentication (JWT & SSO)
Sentinel utilizes JSON Web Tokens (JWT) for session management and integrates with modern identity providers (Okta, Azure AD, Google) via OIDC and SAML.

```toml
[security]
enable_auth = true

[security.sso_provider]
type = "Oidc"
issuer_url = "https://accounts.google.com"
client_id = "..."
client_secret = "..."
```

## 🌐 Global Fleet Management (H100/A100 Clusters)

Sentinel scales from a single server to massive, multi-datacenter GPU fleets via the **Mission Control Plane (MCP)**.

### Single Pane of Glass
Designate a Sentinel node as an **MCP Master**. It aggregates telemetry from all leaf nodes across your datacenters, providing a unified view of:
- **Global PUE & Efficiency**: Real-time cross-cluster power scoring.
- **Unified Topology**: Heatmaps and hardware health for thousands of nodes.
- **Bulk Skill Deployment**: Push autonomic automations (skills) to the entire fleet in a single click.

```toml
# Master Node Configuration
enable_mcp = true

[[orchestrator.clusters]]
name = "US-East-DC-01"
endpoints = ["10.0.1.10:9100", "10.0.1.11:9100"]
```

## 🛡️ Robust Security & Skill Isolation

### 1. Capability-Based WASM Sandboxing
Sentinel WASM Skills (written in **Python, Go, Rust, JS**, etc.) operate under a **Zero-Trust** model.
- **Isolation**: Every skill runs in its own memory cage.
- **Permissions**: Skills must explicitly request capabilities (e.g., `Allow NVML Read`).
- **Signature Verification**: Only cryptographically signed skills from your enterprise CA can be deployed to production nodes.

### 2. Physical-to-Digital Trust
Sentinel uses hardware-backed identity (**TPM/Secure Enclave**) to verify that telemetry is physically sourced from authorized GPU hardware, preventing data spoofing in high-security environments.

---
*ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB*
