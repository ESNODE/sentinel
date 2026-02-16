# ESNODE Sentinel | Source Available BUSL-1.1 | Copyright (c) 2025 Estimatedstocks AB

<div align="center">
  <img src="https://raw.githubusercontent.com/ESNODE/sentinel/main/docs/images/sentinel-logo.png" alt="ESNODE Sentinel - The Sentience Layer for AI Infrastructure" width="600"/>
  
  <h3>The Sentience Layer for AI Infrastructure</h3>
  
  [![License](https://img.shields.io/badge/License-BUSL--1.1-blue.svg)](LICENSE)
  [![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/ESNODE/sentinel)
  [![WASM Powered](https://img.shields.io/badge/Powered_by-WASM-purple.svg)](https://webassembly.org/)
  [![NVIDIA DGX Ready](https://img.shields.io/badge/NVIDIA-DGX_Ready-success)](https://www.nvidia.com/en-us/data-center/dgx-systems/)
  [![Security](https://img.shields.io/badge/Security-Hardened-critical)](SECURITY.md)
  
  <p><i>The world's first power-aware, WASM-extensible observability sentinel for GPU clusters.</i></p>
</div>

---

## ✱ Watch over your AI Fleet

ESNODE Sentinel is a high-performance, GPU-aware observability agent purpose-built for the scale and thermal complexity of modern AI infrastructure. It transforms raw hardware telemetry into **Actionable Sentience**.

### 🦉 Why Sentinel?

*   **⚡ WASM Skills System:** Modular architecture. Drop in new "Skills" (MQTT, SNMP, DNP3, custom hardware) without recompiling the core.
*   **🧠 AIOps Intelligence:** Autonomous Root Cause Analysis (RCA) and Predictive Maintenance built directly into the agent.
*   **🔋 Power-Aware Orchestration:** Real-time PUE calculation and performance-per-watt scoring for green AI.
*   **🖥️ Modern TUI:** A premium, dark-mode terminal interface for instant infrastructure command and control.

---

## 🚀 The Skills System (WASM-Ready)

Sentinel isn't just a tool; it's an extensible platform. Our **Skills System** allows you to adapt Sentinel to any environment.

| Skill | Category | Description |
| :--- | :--- | :--- |
| **GPU Core** | Hardware | NVIDIA/AMD Deep telemetry (NVML/ROCm). |
| **Energy** | Sustainability | Real-time PUE and token-per-watt metrics. |
| **MQTT** | IoT/DCIM | Stream sensor data from Datacenter PDUs. |
| **AIOps** | Intelligence | Autonomous preemption and risk scoring. |

---

## 🛠️ Quick Start

Launch the Sentinel with a single command:

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

## 🏛️ Architecture & Enterprise

ESNODE Sentinel is designed for production reliability:
- **Zero-Dependency Core:** Single static binary for easy deployment.
- **WASM Isolation:** Third-party skills run in a secure sandbox.
- **Enterprise-Grade TUI:** No browser needed for deep diagnostics.

---

## 📄 License & Commercial

ESNODE Sentinel is source-available under the **ESNODE BUSL-1.1** license. 
- **Free for everyone** with fewer than 100 GPUs under management.
- **Enterprise License required** for large-scale GPU fleets and commercial redistribution.

See [LICENSE](LICENSE) for details.

<div align="center">
  <p>Built with ❤️ by the ESNODE Team</p>
</div>
