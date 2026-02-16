extern "C" {
    fn report_metric(name_ptr: i32, name_len: i32, value: f64);
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let mut vec = Vec::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    std::mem::forget(vec);
    ptr
}

#[no_mangle]
pub extern "C" fn init(config_ptr: *const u8, config_len: usize) {
    let _config_str = unsafe {
        let slice = std::slice::from_raw_parts(config_ptr, config_len);
        std::str::from_utf8_unchecked(slice)
    };
    // In a real skill, you'd parse this JSON
}

#[no_mangle]
pub extern "C" fn collect() {
    let metric_name = "wasm_custom_telemetry";
    let value = 42.0; 
    
    unsafe {
        report_metric(metric_name.as_ptr() as i32, metric_name.len() as i32, value);
    }
}
