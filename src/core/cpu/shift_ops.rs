use crate::console_print;
use crate::core::constants::{A, B, C, D, E, H, L};
use crate::core::cpu::CPU;

const HL: usize = 16;

impl CPU{
    pub fn shift_ops(&mut self) -> usize{
        let opcode = self.mem.read_8bit(self.pc+1);
        let mut cycle_count = 8;

        // register can be decoded from first 3 bits of opcode
        let register_index = (opcode & 0x7) as usize;
        // Array to convert from index to register name
        let register_list = [B, C, D, E, H, L, HL, A];
        // Read value from either an 8 bit register, or from memory location at address stored in HL
        let mut value = match register_index{
            HL => self.mem.read_8bit(self.get_hl() as usize),
            r => self.reg[r],
        };
        match opcode{
            0x00..=0x07 => {value = self.rotate_left_circular(value);} // RLC
            0x08..=0x0F => {value = self.rotate_right_circular(value);} // RRC
            0x10..=0x17 => {value = self.rotate_left(value);} // RL
            0x18..=0x1F => {value = self.rotate_right(value);} // RR
            0x20..=0x27 => {} // SLA
            0x28..=0x2F => {} // SRA
            0x30..=0x37 => {} // SWAP
            0x38..=0x3F => {} // SRL
            0x40..=0x7F => {} // BIT
            0x80..=0xBF => {} // RES
            _ => {} // SET
        };


        // Write value to either an 8 bit register, or to memory location at address stored in HL
        match register_index{
            HL => {self.mem.write_8bit(self.get_hl() as usize, value); cycle_count = 16;}
            r => {self.reg[r] = value;}
        };

        self.pc+=2;
        return cycle_count;
    }

    pub fn rotate_left_circular(&mut self, value: u8) -> u8{
        let mut result = value << 1;
        if value & 0x80 !=0{
            result |= 0x1;
            self.set_carry_flag();
        }else{
            result &= 0xFE;
            self.clear_carry_flag();
        }
        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }
        self.clear_sub_flag();
        self.clear_half_carry_flag();

        return result;
    }
    pub fn rotate_right_circular(&mut self, value: u8) -> u8{
        let mut result = value >> 1;

        if value & 0x1 !=0{
            result |= 0x80;
            self.set_carry_flag();
        }else{
            result &= 0x7F;
            self.clear_carry_flag();
        }
        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }
        self.clear_sub_flag();
        self.clear_half_carry_flag();

        return result;
    }
    pub fn rotate_left(&mut self, value: u8) -> u8{
        let mut result = value << 1;
        if self.is_carry_flag_set(){
            result |= 0x1;
        }

        if value & 0x80 !=0{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }
        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }
        self.clear_sub_flag();
        self.clear_half_carry_flag();

        return result;
    }
    pub fn rotate_right(&mut self, value: u8) -> u8{
        let mut result = value >> 1;
        if self.is_carry_flag_set(){
            result |= 0x80;
        }

        if value & 0x1 !=0{
            self.set_carry_flag();
        }else{
            self.clear_carry_flag();
        }
        if result == 0{
            self.set_zero_flag();
        }else{
            self.clear_zero_flag();
        }
        self.clear_sub_flag();
        self.clear_half_carry_flag();

        return result;
    }
}