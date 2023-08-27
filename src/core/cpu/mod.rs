
use crate::core::memory::{init_memory, Memory};

pub struct CPU{
    pub reg: [u8; 8], // 8 8-bit registers
    pub pc: usize, // Program counter
    pub mem: Memory,
}

impl CPU{
    // Initialize CPU
    pub fn init(&mut self){
        self.reg = [0; 8];
        self.pc = 0;
        self.mem = init_memory();
    }
}