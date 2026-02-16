// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use wasmtime::*;
use std::sync::Arc;
use parking_lot::Mutex;
use anyhow::{Context, Result};
use crate::metrics::MetricsRegistry;

pub struct WasmHostState {
    pub metrics: Arc<MetricsRegistry>,
    pub skill_name: String,
}

pub struct WasmRuntime {
    engine: Engine,
    module: Module,
}

impl WasmRuntime {
    pub fn new(wasm_bytes: &[u8]) -> Result<Self> {
        let engine = Engine::default();
        let module = Module::from_binary(&engine, wasm_bytes)
            .context("Failed to compile WASM module")?;
        Ok(Self { engine, module })
    }

    pub fn instantiate(&self, metrics: Arc<MetricsRegistry>, skill_name: String) -> Result<WasmInstance> {
        let mut linker = Linker::new(&self.engine);
        let state = WasmHostState {
            metrics,
            skill_name,
        };
        let mut store = Store::new(&self.engine, state);

        // Define host functions
        linker.func_wrap("env", "report_metric", |mut caller: Caller<'_, WasmHostState>, name_ptr: i32, name_len: i32, value: f64| {
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

    pub fn call_init(&mut self, _config_json: &str) -> Result<()> {
        if let Ok(init) = self.instance.get_typed_func::<(i32, i32), ()>(&mut self.store, "init") {
             // Placeholder for config passing
             tracing::debug!("WASM init called (config passing placeholder)");
             let _ = init;
        }
        Ok(())
    }
}
