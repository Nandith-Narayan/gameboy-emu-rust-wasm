mod execute;

use crate::console_print;
use crate::core::constants::{A, B, C, D, E, F, H, L};
use crate::core::memory::{init_memory, Memory};

pub struct CPU{
    pub reg: [u8; 8], // 8 8-bit registers
    pub pc: usize, // Program counter
    pub sp: usize, // Stack pointer
    pub mem: Memory,
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
        self.pc = 0x101;
        self.sp = 0xFFFE;
        self.mem = init_memory();
    }

    // Helper functions to read and write 16-Bit registers
    pub fn get_hl(&self) -> u16{
        return ((self.reg[H] as u16)<<8).wrapping_add(self.reg[L] as u16);
    }
    pub fn set_hl(&mut self, value: u16){
        self.reg[H] = ((value >> 8) & 0x0FF) as u8;
        self.reg[L] = (value & 0x0FF) as u8;
    }
    pub fn get_bc(&self) -> u16{
        return ((self.reg[B] as u16)<<8).wrapping_add(self.reg[C] as u16);
    }
    pub fn set_bc(&mut self, value: u16){
        self.reg[B] = ((value >> 8) & 0x0FF) as u8;
        self.reg[C] = (value & 0x0FF) as u8;
    }
    pub fn get_de(&self) -> u16{
        return ((self.reg[D] as u16)<<8).wrapping_add(self.reg[E] as u16);
    }
    pub fn set_de(&mut self, value: u16){
        self.reg[D] = ((value >> 8) & 0x0FF) as u8;
        self.reg[E] = (value & 0x0FF) as u8;
    }
    pub fn get_af(&self) -> u16{
        return ((self.reg[A] as u16)<<8).wrapping_add(self.reg[F] as u16);
    }
    pub fn set_af(&mut self, value: u16){
        self.reg[A] = ((value >> 8) & 0x0FF) as u8;
        self.reg[F] = (value & 0x0FF) as u8;
    }

    // Helper functions to increment and decrement 16-Bit registers
    pub fn inc_hl(&mut self){
        let val = self.get_hl();
        self.set_hl(val.wrapping_add(1));
    }
    pub fn inc_bc(&mut self){
        let val = self.get_bc();
        self.set_bc(val.wrapping_add(1));
    }
    pub fn inc_de(&mut self){
        let val = self.get_de();
        self.set_de(val.wrapping_add(1));
    }
    pub fn dec_hl(&mut self){
        let val = self.get_hl();
        self.set_hl(val.wrapping_sub(1));
    }
    pub fn dec_bc(&mut self){
        let val = self.get_bc();
        self.set_bc(val.wrapping_sub(1));
    }
    pub fn dec_de(&mut self){
        let val = self.get_de();
        self.set_de(val.wrapping_sub(1));
    }

    // Helper functions to set and clear CPU flags
    pub fn set_carry_flag(&mut self){
        self.reg[F] |= 0b00010000;
    }
    pub fn clear_carry_flag(&mut self){
        self.reg[F] &= 0b11100000;
    }
    pub fn set_half_carry_flag(&mut self){
        self.reg[F] |= 0b00100000;
    }
    pub fn clear_half_carry_flag(&mut self){
        self.reg[F] &= 0b11010000;
    }
    pub fn set_sub_flag(&mut self){
        self.reg[F] |= 0b01000000;
    }
    pub fn clear_sub_flag(&mut self){
        self.reg[F] &= 0b10110000;
    }
    pub fn set_zero_flag(&mut self){
        self.reg[F] |= 0b10000000;
    }
    pub fn clear_zero_flag(&mut self){
        self.reg[F] &= 0b01110000;
    }
    pub fn is_zero_flag_set(&self) -> bool{
        return (self.reg[F] & 0b10000000) != 0;
    }
    pub fn is_sub_flag_set(&self) -> bool{
        return (self.reg[F] & 0b01000000) != 0;
    }
    pub fn is_half_carry_flag_set(&self) -> bool{
        return (self.reg[F] & 0b00100000) != 0;
    }
    pub fn is_carry_flag_set(&self) -> bool{
        return (self.reg[F] & 0b00010000) != 0;
    }

    // Helper functions to perform math operations
    pub fn sub_and_set_flags(&mut self, a: u8, b: u8) -> u8{

        let result = a.wrapping_sub(b);

        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }

        self.set_sub_flag();

        if (a ^ b ^ result) & 0x10 == 0x10{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        if ((a as u16) ^ (b as u16) ^ ((a as u16).wrapping_sub(b as u16))) & 0x100 == 0x100{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }


        return result;
    }
}