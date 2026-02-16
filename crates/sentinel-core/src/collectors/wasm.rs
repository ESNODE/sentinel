// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use async_trait::async_trait;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::collectors::Collector;
use crate::metrics::MetricsRegistry;
use crate::skills::wasm_runtime::{WasmRuntime, WasmInstance};

pub struct WasmCollector {
    name: String,
    instance: Arc<Mutex<WasmInstance>>,
}

impl WasmCollector {
    pub fn new(name: String, wasm_bytes: &[u8], metrics: Arc<MetricsRegistry>, config_json: &str) -> anyhow::Result<Self> {
        let runtime = WasmRuntime::new(wasm_bytes)?;
        let mut instance = runtime.instantiate(metrics, name.clone())?;
        
        instance.call_init(config_json)?;
        
        Ok(Self {
            name,
            instance: Arc::new(Mutex::new(instance)),
        })
    }
}

#[async_trait]
impl Collector for WasmCollector {
    fn name(&self) -> &'static str {
        // For dynamic names, we leak once or keep a static map.
        // Since collectors are typically long-lived, this is acceptable for now.
        Box::leak(self.name.clone().into_boxed_str())
    }

    async fn collect(&mut self, _metrics: &MetricsRegistry) -> anyhow::Result<()> {
        let mut instance = self.instance.lock();
        instance.call_collect()?;
        Ok(())
    }
}
