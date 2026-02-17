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

### Authentication (JWT)
Sentinel utilizes JSON Web Tokens (JWT) for session management. When `enable_auth` is true, all management and status APIs require a valid Bearer token.

```toml
[security]
enable_auth = true
```

### SSO (Single Sign-On)
Sentinel integrates with modern identity providers via OIDC and SAML.

#### OIDC Configuration
```toml
[security.sso_provider]
type = "Oidc"
issuer_url = "https://accounts.google.com"
client_id = "..."
client_secret = "..."
```

#### SAML Configuration
```toml
[security.sso_provider]
type = "Saml"
metadata_url = "https://identity-provider.com/saml/metadata"
```

## 🛠️ Configuration Reference

| Key | Type | Description |
| :--- | :--- | :--- |
| `security.enable_https` | bool | Enables HTTPS listener. |
| `security.cert_path` | string | Path to SSL/TLS certificate. |
| `security.key_path` | string | Path to SSL/TLS private key. |
| `security.enable_auth` | bool | Enables JWT-based authentication for APIs. |
| `security.sso_provider` | object | SSO provider configuration (OIDC/SAML). |

---
*ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB*
