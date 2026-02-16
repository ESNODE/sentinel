# 🧩 Sentinel Skills

Sentinel is built on a **Modular Skill Architecture**. While the core handles the performance-critical engine, **Skills** extend its reach to every corner of your infrastructure.

## 🚀 Why Skills?
*   **Isolation**: Skills run in a WASM sandbox. If a skill crashes, Sentinel keeps watching.
*   **Language Agnostic**: Write skills in Rust, Go, Zig, or C++.
*   **Hot Reloading**: Drop a `.wasm` file in, and it's live. No restarts. No downtime.

## 📦 Core Skills
These are built-in or officially supported:
- **`gpu-nvml`**: High-frequency NVIDIA telemetry.
- **`pue-calc`**: Real-time Data Center efficiency scoring.
- **`protocol-hub`**: Native support for MQTT, Modbus, SNMP.

## 🛠️ Build Your Own
Want to monitor a specific liquid cooling loop or a proprietary PDU? 
Check our [WASM SDK Guide](../docs/WASM_SKILLS.md) to get started.

---
<div align="center">
  <p>Become a <b>Sentinel Sentinel</b>. Contribute to the community.</p>
</div>
