// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use async_trait::async_trait;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::collectors::Collector;
use crate::metrics::MetricsRegistry;
use crate::skills::wasm_runtime::{WasmRuntime, WasmInstance};

pub struct WasmCollector {
    name: &'static str,
    instance: Arc<Mutex<WasmInstance>>,
}

impl WasmCollector {
    pub fn new(name: String, wasm_bytes: &[u8], metrics: Arc<MetricsRegistry>, config_json: &str) -> anyhow::Result<Self> {
        let name_static = Box::leak(name.into_boxed_str()) as &'static str;
        let runtime = WasmRuntime::new(wasm_bytes)?;
        let mut instance = runtime.instantiate(metrics, name_static.to_string())?;
        
        instance.call_init(config_json)?;
        
        Ok(Self {
            name: name_static,
            instance: Arc::new(Mutex::new(instance)),
        })
    }
}

#[async_trait]
impl Collector for WasmCollector {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn collect(&mut self, _metrics: &MetricsRegistry) -> anyhow::Result<()> {
        let mut instance = self.instance.lock();
        instance.call_collect()?;
        Ok(())
    }
}
