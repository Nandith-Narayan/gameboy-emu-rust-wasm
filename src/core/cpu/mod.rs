mod execute;
mod shift_ops;
mod cpu_helper_functions;
mod interrupt_handler;

use crate::console_print;
use crate::core::constants::{A, B, C, D, E, F, H, L};
use crate::core::memory::{init_memory, Memory};
use crate::core::ppu::{init_ppu, PPU};

pub struct CPU{
    pub reg: [u8; 8], // 8 8-bit registers
    pub pc: usize, // Program counter
    pub sp: usize, // Stack pointer
    pub interrupt_master_enable: bool,
    pub enable_interrupt_next_instruction: bool,
    pub mem: Memory,
    pub ppu: PPU,
    pub frame_done: bool,
    pub unique_ops: Vec<u8>,


    pub total_cycles: u64,
}

impl CPU{
    // Initialize CPU
    pub fn init(&mut self){
        self.reg = [0; 8];
        self.reg[A] = 0x1;
        self.reg[C] = 0x13;
        self.reg[E] = 0xD8;
        self.reg[H] = 0x1;
        self.reg[L] = 0x4D;
        self.reg[F] = 0xB0;
        self.pc = 0x100;
        self.sp = 0xFFFE;
        self.interrupt_master_enable = false;
        self.enable_interrupt_next_instruction = false;
        self.mem = init_memory();
        self.ppu = init_ppu();
        self.frame_done = false;
        self.unique_ops = vec![];
        self.total_cycles = 0;
    }


    pub fn render_background_tile_data(&mut self) -> Vec<u8>{
        return self.ppu.render_background_tile_data(&mut self.mem);
    }
}