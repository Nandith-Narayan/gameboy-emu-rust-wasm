use crate::core::cpu::CPU;
use crate::core::constants::{A, B, C, D, E, F, H, L};

impl CPU{
    // Helper functions to read and write 16-Bit registers
    pub fn get_hl(&self) -> u16{
        return ((self.reg[H] as u16)<<8).wrapping_add(self.reg[L] as u16);
    }
    pub fn set_hl(&mut self, value: u16){
        self.reg[H] = ((value >> 8) & 0x0FF) as u8;
        self.reg[L] = (value & 0x0FF) as u8;
    }
    pub fn get_bc(&self) -> u16{
        return ((self.reg[B] as u16)<<8) + (self.reg[C] as u16);
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
        self.reg[F] = (value & 0x0F0) as u8; // Lower bits of flags register must be 0
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
    pub fn sub_with_carry_and_set_flags(&mut self, a: u8, b: u8) -> u8{
        let carry_val = if self.is_carry_flag_set(){1}else{0};
        let result = a.wrapping_sub(b).wrapping_sub(carry_val);

        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }

        self.set_sub_flag();

        if (a ^ b ^ result ^ carry_val) & 0x10 == 0x10{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        if ((a as u16) ^ (b as u16) ^ (carry_val as u16) ^ ((a as u16).wrapping_sub(b as u16).wrapping_sub(carry_val as u16))) & 0x100 == 0x100{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }


        return result;
    }
    pub fn add_and_set_flags(&mut self, a: u8, b: u8) -> u8{

        let result = a.wrapping_add(b);

        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }

        self.clear_sub_flag();

        if (a ^ b ^ result) & 0x10 == 0x10{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        if ((a as u16) ^ (b as u16) ^ ((a as u16).wrapping_add(b as u16))) & 0x100 == 0x100{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }


        return result;
    }
    pub fn add_with_carry_and_set_flags(&mut self, a: u8, b: u8) -> u8{
        let carry_val = if self.is_carry_flag_set(){1}else{0};
        let result = a.wrapping_add(b).wrapping_add(carry_val);

        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }

        self.clear_sub_flag();

        if (a ^ b ^ result ^ carry_val) & 0x10 == 0x10{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        if ((a as u16) ^ (b as u16) ^ (carry_val as u16)  ^ ((a as u16).wrapping_add(b as u16).wrapping_add(carry_val as u16))) & 0x100 == 0x100{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }


        return result;
    }
    pub fn add_to_hl_and_set_flags(&mut self, value: u16){
        let a = self.get_hl() as usize;
        let b = value as usize;
        let result = self.get_hl().wrapping_add(value);
        if (((a & 0xFFF) + (b&0xFFF))&0x1000) != 0 {
            self.set_half_carry_flag();
        }else {
            self.clear_half_carry_flag();
        }
        // set Carry Flag
        if (((a & 0xFFFF) + (b&0xFFFF))&0x10000) != 0 {
            self.set_carry_flag();
        }else {
            self.clear_carry_flag();
        }
        self.clear_sub_flag();
        self.set_hl(result);
    }
    pub fn daa(&mut self){
        if !self.is_sub_flag_set() {
            if self.is_carry_flag_set() || self.reg[A] > 0x99 {
                self.reg[A] =self.reg[A].wrapping_add(0x60);
                self.set_carry_flag();
            }
            if self.is_half_carry_flag_set() || (self.reg[A] & 0xF) > 0x9 {
                self.reg[A] = self.reg[A].wrapping_add(0x6);
            }
        } else {  // after a subtraction, only adjust if (half-)carry occurred
            if self.is_carry_flag_set() {
                self.reg[A] = self.reg[A].wrapping_sub(0x60);
            }
            if self.is_half_carry_flag_set() {
                self.reg[A] = self.reg[A].wrapping_sub(0x6);
            }
        }
        if self.reg[A] == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }
        self.clear_half_carry_flag();
    }
    pub fn load_hl_with_sp_plus_r8(&mut self){
        let val = self.mem.read_8bit(self.pc + 1) as i8;


        // Compute carry and half-carry flags
        if ((self.sp & 0xFF) + ((val as usize) & 0xFF)) & 0x100 != 0{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }
        if ((self.sp & 0xF) + ((val as usize) & 0xF)) & 0x10 != 0{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        self.set_hl((self.sp as i32 + (val as i32)) as u16);

        self.clear_zero_flag();
        self.clear_sub_flag();
    }
    pub fn sp_plus_r8(&mut self){
        let val = self.mem.read_8bit(self.pc + 1) as i8;


        // Compute carry and half-carry flags
        if ((self.sp & 0xFF) + ((val as usize) & 0xFF)) & 0x100 != 0{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }
        if ((self.sp & 0xF) + ((val as usize) & 0xF)) & 0x10 != 0{
            self.set_half_carry_flag();
        }else{
            self.clear_half_carry_flag();
        }

        self.sp = (self.sp as i32 + val as i32) as usize;

        self.clear_zero_flag();
        self.clear_sub_flag();
    }
}