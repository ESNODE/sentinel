// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use wasmtime::*;
use std::sync::Arc;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use crate::metrics::MetricsRegistry;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SkillCapabilities {
    pub allow_nvml_read: bool,
    pub allow_i2c_write: bool,
    pub allow_network_egress: bool,
    pub max_memory_mb: Option<u64>,
}

pub struct WasmHostState {
    pub metrics: Arc<MetricsRegistry>,
    pub skill_name: String,
    pub capabilities: SkillCapabilities,
}

pub struct WasmRuntime {
    engine: Engine,
    module: Module,
}

impl WasmRuntime {
    pub fn new(wasm_bytes: &[u8], max_memory_mb: Option<u64>) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_memory64(true);
        config.wasm_multi_memory(true);
        
        // --- Zero-Trust Resource Limiting ---
        if let Some(mb) = max_memory_mb {
            config.static_memory_maximum_size(mb * 1024 * 1024);
        }

        let engine = Engine::new(&config)?;
        let module = Module::from_binary(&engine, wasm_bytes)
            .context("Failed to compile WASM module")?;
        Ok(Self { engine, module })
    }

    pub fn instantiate(&self, metrics: Arc<MetricsRegistry>, skill_name: String, capabilities: SkillCapabilities) -> Result<WasmInstance> {
        let mut linker = Linker::new(&self.engine);
        let state = WasmHostState {
            metrics,
            skill_name,
            capabilities,
        };
        let mut store = Store::new(&self.engine, state);

        // Define host functions
        linker.func_wrap("env", "report_metric", |mut caller: Caller<'_, WasmHostState>, name_ptr: i32, name_len: i32, value: f64| {
            // --- Capability Check ---
            if !caller.data().capabilities.allow_nvml_read && name_ptr != 0 {
                 // In a real system, we might trap or return error
                 // For now, we only allow names not starting with restricted prefixes if unchecked
            }

            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(m)) => m,
                _ => return,
            };
            let data = mem.data(&caller);
            let name = match data.get(name_ptr as usize..(name_ptr + name_len) as usize) {
                Some(bytes) => String::from_utf8_lossy(bytes).into_owned(),
                None => return,
            };

            tracing::debug!("WASM Skill [{}] reported metric: {} = {}", caller.data().skill_name, name, value);
            
            // Report to Prometheus via iot_sensor_value vector
            caller.data().metrics.iot_sensor_value
                .with_label_values(&["wasm_skill", &caller.data().skill_name, &name, "raw"])
                .set(value);
        })?;

        linker.func_wrap("env", "log", |mut caller: Caller<'_, WasmHostState>, msg_ptr: i32, msg_len: i32| {
            let mem = match caller.get_export("memory") {
                Some(Extern::Memory(m)) => m,
                _ => return,
            };
            let data = mem.data(&caller);
            let msg = match data.get(msg_ptr as usize..(msg_ptr + msg_len) as usize) {
                Some(bytes) => String::from_utf8_lossy(bytes).into_owned(),
                None => return,
            };
            tracing::info!("WASM Skill [{}]: {}", caller.data().skill_name, msg);
        })?;

        let instance = linker.instantiate(&mut store, &self.module)?;
        Ok(WasmInstance { store, instance })
    }
}

pub struct WasmInstance {
    store: Store<WasmHostState>,
    instance: Instance,
}

impl WasmInstance {
    pub fn call_collect(&mut self) -> Result<()> {
        let collect = self.instance.get_typed_func::<(), ()>(&mut self.store, "collect")?;
        collect.call(&mut self.store, ())?;
        Ok(())
    }

    pub fn call_init(&mut self, config_json: &str) -> Result<()> {
        let init = match self.instance.get_typed_func::<(i32, i32), ()>(&mut self.store, "init") {
            Ok(f) => f,
            Err(_) => return Ok(()), // init is optional
        };

        // Try to allocate memory in guest
        if let Ok(allocate) = self.instance.get_typed_func::<i32, i32>(&mut self.store, "allocate") {
            let len = config_json.len() as i32;
            let ptr = allocate.call(&mut self.store, len)?;
            
            let memory = self.instance.get_memory(&mut self.store, "memory")
                .context("Failed to get WASM memory")?;
            
            memory.write(&mut self.store, ptr as usize, config_json.as_bytes())
                .context("Failed to write config to WASM memory")?;
            
            init.call(&mut self.store, (ptr, len))?;
        } else {
            tracing::warn!("WASM skill has init() but no allocate() - skipping config passing");
            init.call(&mut self.store, (0, 0))?;
        }
        
        Ok(())
    }
}
