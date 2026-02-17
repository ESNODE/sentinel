# Sentinel

<div align="center">
  <img src="https://raw.githubusercontent.com/ESNODE/sentinel/main/docs/images/sentinel-logo.png" alt="Sentinel Logo" width="600"/>
  
  <h3>The Sentience Layer for AI Infrastructure</h3>
  
  **Stop observing. Start Sensing.** Sentinel is the first high-performance, **WASM-extensible** observability framework purpose-built for the scale, thermal complexity, and power-density of modern GPU clusters.

  [**Explore the Docs**](./docs/) | [**Build a Skill**](./skills/README.md)

  [![License](https://img.shields.io/badge/License-BUSL--1.1-blue.svg)](LICENSE)
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/ESNODE/sentinel)
  [![WASM Powered](https://img.shields.io/badge/Powered_by-WASM-purple.svg)](https://webassembly.org/)
  [![NVIDIA DGX Ready](https://img.shields.io/badge/NVIDIA-DGX_Ready-success)](https://www.nvidia.com/en-us/data-center/dgx-systems/)
  [![Go Support](https://img.shields.io/badge/Go-Ready-blue.svg)](https://go.dev/)
  [![Hype](https://img.shields.io/badge/Hype-Extreme-orange.svg)](https://github.com/ESNODE/sentinel)
</div>

---

## Visual Showcase

<div align="center">
  <h3>The "Aha!" Moment: Advanced RCA in the TUI</h3>
  <img src="https://raw.githubusercontent.com/ESNODE/sentinel/main/docs/images/sentinel-hero.png" alt="Sentinel Hero Visual" width="800"/>
  <p><i>Watch Sentinel detect thermal throttling and automatically mitigate it via the Fan Control Skill in real-time.</i></p>

  <br/>

  <h3>The "WASM Magic": Polyglot Extensibility</h3>
  <img src="https://raw.githubusercontent.com/ESNODE/sentinel/main/docs/images/wasm-magic.png" alt="Sentinel WASM Magic" width="800"/>
  <p><i>Build a Skill in <b>Python, JavaScript, TypeScript, Rust, Go, C++, C#, Java</b>, or <b>Kotlin</b>. Compile to WASM and see it live instantly.</i></p>
</div>

---

## ⚡ 60-Second Full Stack Demo

Experience the full power of Sentinel + Prometheus + Grafana with a single command.

```bash
# Launch the full stack: Sentinel + Prometheus + Grafana
docker compose -f examples/full-stack/docker-compose.yml up -d
```

**What happens?**
1.  **Sentinel** starts sensing your host (GPU, CPU, Power).
2.  **Prometheus** begins scraping Sentinel every 5s.
3.  **Grafana** is provisioned with an automatic datasource.
4.  **Visit**: [http://localhost:3000](http://localhost:3000) (Admin/Admin) to see your real-time AI infra health.

---

## 🔭 The Vision

Sentinel is not just a tool; it is a movement toward **Autonomic AI Infrastructure**. Read our clear vision for the next era of computing:

👉 [**Sentinel Vision 2026: The Sentience Layer**](./docs/VISION_2026.md)

---

## Watch over your AI Fleet

ESNODE Sentinel is a high-performance, GPU-aware observability agent purpose-built for the scale and thermal complexity of modern AI infrastructure. It transforms raw hardware telemetry into **Actionable Sentience**.

### Why Sentinel?

*   **Polyglot Skills System:** Modular architecture. Build "Skills" in **Python, JavaScript, TypeScript, Rust, Go, C++, C#, Java, or Kotlin**. Drop them in as WASM binaries without recompiling the core.
*   **AIOps Intelligence:** Autonomous Root Cause Analysis (RCA) and Predictive Maintenance built directly into the agent.
*   **Power-Aware Orchestration:** Real-time PUE calculation and performance-per-watt scoring for green AI.
*   **Modern TUI:** A premium, dark-mode terminal interface for instant infrastructure command and control.

---

## The WASM Skills Gallery

Traditional agents are rigid. Sentinel is a living organism. Extensions are **Polyglot**—write in your language of choice and compile to **WebAssembly**.

| Skill | Category | Capability | Viral Factor |
| :--- | :--- | :--- | :--- |
| **GPU-Sentience** | Core | NVML/ROCm Deep-packet telemetry. | **High** |
| **Eco-Flow** | Energy | Real-time PUE & Token/Joule Efficiency. | **Hype** |
| **IoT-Bridge** | Protocol | MQTT/DNP3 integration for Liquid Cooling. | **Essential** |
| **Risk-Oracle** | AIOps | Predictive GPU Failure Scoring (0-100). | **Magic** |

> **Contribute a Skill:** Get listed in the [Community Gallery](skills/COMMUNITY.md) and earn exclusive **Sentinel Sentinel** status.

---

## 🛠️ Quick Start

Launch Sentinel with a single command:

```bash
# Clone the repository
git clone https://github.com/ESNODE/sentinel.git
cd sentinel

# Build and run
cargo run --release --bin esnode-sentinel
```

### ⌨️ Launch the TUI
```bash
esnode-sentinel cli
```

---

## 📊 Modern Observability. Zero Configuration.

Sentinel exports a rich Prometheus-compatible metrics stream at `:9100/metrics` and provides a real-time JSON status API at `/status`.

```json
{
  "healthy": true,
  "load_avg_1m": 0.15,
  "gpus": [
    {
      "id": "GPU 0",
      "util": 88.5,
      "power": 320.4,
      "risk_score": 12.0
    }
  ]
}
```

---

<div align="center">
  <p>Built for the scale of tomorrow. 🦅</p>
  <p>ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB</p>
</div>

---

## 🏛️ Architecture & Enterprise

ESNODE Sentinel is designed for production reliability:
- **Zero-Dependency Core:** Single static binary for easy deployment.
- **WASM Isolation:** Third-party skills run in a secure sandbox.
- **Enterprise-Grade TUI:** No browser needed for deep diagnostics.
- **Security & Cloud Console:** Full [Security & Console Guide](./docs/SECURITY_CONSOLE.md) for HTTPS and SSO.
- **Strategic Adoption:** Read our [Enterprise & Mega-Cap Strategy](./docs/enterprise/) for large-scale fleet integration.

---

## 📄 License & Commercial

ESNODE Sentinel is source-available under the **ESNODE BUSL-1.1** license. 
- **Free for everyone** with fewer than 100 GPUs under management.
- **Enterprise License required** for large-scale GPU fleets and commercial redistribution.

See [LICENSE](LICENSE) for details.

<div align="center">
  <p>Built with ❤️ by the ESNODE Team</p>
</div>
