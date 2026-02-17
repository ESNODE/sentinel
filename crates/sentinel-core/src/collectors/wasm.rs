// ESNODE | Source Available BUSL-1.1 | Copyright (c) 2024 Estimatedstocks AB
use async_trait::async_trait;
use std::sync::Arc;
use parking_lot::Mutex;
use crate::collectors::Collector;
use crate::metrics::MetricsRegistry;
use crate::skills::wasm_runtime::{WasmRuntime, WasmInstance, SkillCapabilities};

pub struct WasmCollector {
    name: &'static str,
    instance: Arc<Mutex<WasmInstance>>,
}

impl WasmCollector {
    pub fn new(
        name: String, 
        wasm_bytes: &[u8], 
        metrics: Arc<MetricsRegistry>, 
        config_json: &str,
        capabilities: SkillCapabilities,
        signature: Option<String>,
        public_key: Option<String>,
    ) -> anyhow::Result<Self> {
        // --- Enterprise Security: Signature Verification ---
        if let (Some(sig), Some(pk)) = (signature, public_key) {
            crate::auth::verify_skill_signature(wasm_bytes, &sig, &pk)?;
            tracing::info!("Skill [{}] signature verified successfully.", name);
        }

        let name_static = Box::leak(name.into_boxed_str()) as &'static str;
        let runtime = WasmRuntime::new(wasm_bytes, capabilities.max_memory_mb)?;
        let mut instance = runtime.instantiate(metrics, name_static.to_string(), capabilities)?;
        
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
