extern "C" {
    fn report_metric(name_ptr: i32, name_len: i32, value: f64);
}

#[no_mangle]
pub extern "C" fn collect() {
    let metric_name = "wasm_custom_telemetry";
    let value = 42.0; 
    
    unsafe {
        report_metric(metric_name.as_ptr() as i32, metric_name.len() as i32, value);
    }
}
