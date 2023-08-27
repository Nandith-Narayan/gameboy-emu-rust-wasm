mod utils;
mod core;

use std::panic;
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
pub fn initialize_rom(rom: Vec<u8>) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut gb_cpu = GB_CPU.lock().unwrap();
    gb_cpu.init();

    gb_cpu.mem.rom = vec![0; 0x40000];

    // Load ROM data into memory
    for i in 0..rom.len(){
        gb_cpu.mem.rom[i] = rom[i];
    }
}



#[wasm_bindgen]
pub fn run() {
    let mut gb_cpu = GB_CPU.lock().unwrap();

    for i in 0..100usize{
        gb_cpu.execute();
    }

    //alert("Hello, World!");
}
