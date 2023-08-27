mod instruction;
mod execute;

use crate::core::constants::{B, C, D, E, H, L};
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

    // Helper functions to read and write 16-Bit registers
    pub fn get_hl(&self) -> u16{
        return (self.reg[H] as u16)<<8 + (self.reg[L] as u16);
    }
    pub fn set_hl(&mut self, value: u16){
        self.reg[H] = ((value << 8) & 0x0FF) as u8;
        self.reg[L] = (value & 0x0FF) as u8;
    }
    pub fn get_bc(&self) -> u16{
        return (self.reg[B] as u16)<<8 + (self.reg[C] as u16);
    }
    pub fn set_bc(&mut self, value: u16){
        self.reg[B] = ((value << 8) & 0x0FF) as u8;
        self.reg[C] = (value & 0x0FF) as u8;
    }
    pub fn get_de(&self) -> u16{
        return (self.reg[D] as u16)<<8 + (self.reg[E] as u16);
    }
    pub fn set_de(&mut self, value: u16){
        self.reg[D] = ((value << 8) & 0x0FF) as u8;
        self.reg[E] = (value & 0x0FF) as u8;
    }
}