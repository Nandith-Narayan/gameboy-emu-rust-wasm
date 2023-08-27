mod utils;
mod core;

use std::sync::Mutex;
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use crate::core::cpu::CPU;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref GB_CPU: Mutex<CPU> = Mutex::new(CPU{ reg: [0; 8], pc: 0, mem: core::memory::init_memory() });
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let mut gb_cpu = GB_CPU.lock().unwrap();
    gb_cpu.init();
    alert("Hello, World!");
}
