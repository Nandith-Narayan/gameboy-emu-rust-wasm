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
    static ref GB_CPU: Mutex<CPU> = Mutex::new(CPU{ reg: [0; 8], pc: 0, sp: 0, interrupt_master_enable: false, enable_interrupt_next_instruction: false, mem: core::memory::init_memory(), ppu: core::ppu::init_ppu(), frame_done: false, unique_ops:vec![0], total_cycles:0});
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_print(s: &str);
}

#[wasm_bindgen]
pub fn initialize_rom(rom: Vec<u8>) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let mut gb_cpu = GB_CPU.lock().unwrap();
    gb_cpu.init();

    gb_cpu.mem.rom = vec![0; 0x40000*16];

    // Load ROM data into memory
    for i in 0..rom.len(){
        gb_cpu.mem.rom[i] = rom[i];
    }
}



#[wasm_bindgen]
pub fn run_until_frame_end() -> Vec<u8> {
    let mut gb_cpu = GB_CPU.lock().unwrap();
    gb_cpu.frame_done = false;
    while !gb_cpu.frame_done{
        gb_cpu.execute();
    }
    //console_print(gb_cpu.mem.debug_string.as_str());
    //console_print("done");
    //console_print(format!("{:?}", gb_cpu.unique_ops).as_str());
    //alert("Hello, World!");

    return gb_cpu.ppu.frame_buffer.clone();
}
#[wasm_bindgen]
pub fn get_background_tile_data() -> Vec<u8> {
    let mut gb_cpu = GB_CPU.lock().unwrap();

    return gb_cpu.render_background_tile_data();
}