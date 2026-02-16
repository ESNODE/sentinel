# Sentinel Skills

Sentinel is built on a **Modular Skill Architecture**. While the core handles the performance-critical engine, **Skills** extend its reach to every corner of your infrastructure.

## Why Skills?
*   **Isolation**: Skills run in a WASM sandbox. If a skill crashes, Sentinel keeps watching.
*   **Maximum Accessibility**: Write skills in **Python, JavaScript, TypeScript, Rust, Go, C++, C#, Java,** or **Kotlin**.
*   **Hot Reloading**: Drop a `.wasm` file in, and it's live. No restarts. No downtime.

## The Polyglot Ecosystem
Sentinel utilizes the **WebAssembly (WASM)** standard to support almost any modern programming language:
*   **Web-First**: Full support for **JavaScript** and **TypeScript** (via Javy/QuickJS).
*   **Scripting**: High-performance **Python** integration.
*   **Enterprise**: Build industrial-grade skills in **C#, Java,** or **Kotlin** (via TeaVM/GraalVM WASM).
*   **Systems**: Near-native performance with **Rust, Go, C++,** and **Zig**.

## Core Skills
These are built-in or officially supported:
- **`gpu-nvml`**: High-frequency NVIDIA telemetry.
- **`pue-calc`**: Real-time Data Center efficiency scoring.
- **`protocol-hub`**: Native support for MQTT, Modbus, SNMP.

## Build Your Own
Want to monitor a specific liquid cooling loop or a proprietary PDU? 
Check our [WASM SDK Guide](../docs/WASM_SKILLS.md) to get started.

---
<div align="center">
  <p>Become a <b>Sentinel Sentinel</b>. Contribute to the community.</p>
</div>
