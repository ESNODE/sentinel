# 🧩 Building WASM Skills for Sentinel

Sentinel is a **Polyglot** platform. It allows you to extend the observability of your AI infrastructure by writing "Skills" in almost any programming language, compiling them to WebAssembly (WASM), and injecting them into the Sentinel runtime.

## 🚀 The WASM Advantage
- **Zero-Trust Isolation**: Skills run in a secure sandbox. They cannot access your host OS or filesystem without explicit permission.
- **Language Independence**: Use the tools you already know: **Python, JavaScript, Rust, C#, Java, Go, C++, Zig**.
- **Dynamic Injection**: Add or update skills without restarting the Sentinel agent.

---

## 🏗️ Architecture

Every Sentinel Skill is a WASI-compatible WASM module that exports specific functions and interacts with the Sentinel host via an ABI (Application Binary Interface).

### Required Exports
The WASM module MUST export these functions:
- `allocate(len: i32) -> i32`: Called by the host to allocate memory in the guest for configuration passing.
- `init(ptr: i32, len: i32)`: (Optional) Called once upon loading. Receives the configuration JSON.
- `collect()`: Called by the host at every scrape interval. This is where your logic lives.

### Host Imports
Skills can report data back to Sentinel using these host-defined functions:
- `report_metric(name_ptr: i32, name_len: i32, value: f64)`: Reports an IoT sensor metric.
- `log(msg_ptr: i32, msg_len: i32)`: Sends a log message to the Sentinel log stream.

---

## 🐍 Python Example (via Javy)

1. Write your logic in `skill.py`:
   ```python
   def collect():
       # Your logic here
       print("Reporting metric from Python!")
       # Use the Sentinel bridge to report
       # report_metric("gpu_custom_temp", 42.0)
   ```
2. Compile to WASM using [Javy](https://github.com/bytecodealliance/javy):
   ```bash
   javy compile skill.py -o skill.wasm
   ```

## 🟨 JavaScript/TypeScript Example

1. Create `index.js`:
   ```javascript
   export function collect() {
       // Report a value to the Sentinel TUI
       Host.report_metric("liquid_cooling_inlet", 22.5);
   }
   ```
2. Build:
   ```bash
   javy compile index.js -o skill.wasm
   ```

## 🦀 Rust Example

1. Use the `no_mangle` attribute:
   ```rust
   #[no_mangle]
   pub extern "C" fn collect() {
       unsafe {
           report_metric("power_efficiency_score", 0.98);
       }
   }

   extern "C" {
       fn report_metric(ptr: *const u8, len: usize, value: f64);
   }
   ```
2. Build for WASM:
   ```bash
   cargo build --target wasm32-unknown-unknown --release
   ```

---

## 🔧 Deploying Your Skill

1. Place your compiled `.wasm` file into the `skills/` directory of your Sentinel installation.
2. Update your `sentinel.yaml` (or CLI flags) to include the new skill:
   ```yaml
   wasm_skills:
     - name: "my-custom-sensor"
       path: "./skills/my_skill.wasm"
       config:
         threshold: 85.0
   ```
3. Sentinel will automatically load the skill and start the collection loop.

## 🌈 Enterprise Languages (C#, Java, Kotlin)
For C#, Java, or Kotlin, use the following WASM toolchains to generate WASI-compliant binaries:
- **C#**: [NativeAOT-LLVM](https://github.com/dotnet/runtimelab/tree/feature/NativeAOT-LLVM) or [Wasmbuild](https://github.com/v8/v8/wiki/WebAssembly-and-C-Sharp).
- **Java/Kotlin**: [TeaVM](https://teavm.org/) or [GraalVM WASM](https://www.graalvm.org/latest/reference-manual/wasm/).

---

<div align="center">
  <p>Need help? Join the <a href="https://discord.gg/sentinel">Sentinel Discord</a> or check out the <a href="../skills/wasm_sample">Sample Rust Skill</a>.</p>
</div>
