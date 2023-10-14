use std::fmt::format;
use crate::console_print;
use crate::core::constants::{A, B, C, D, E, F, H, L};
use crate::core::cpu::CPU;

impl CPU{
    // Executes the next instruction and returns number of CPU cycles executed
    pub fn execute(&mut self) -> usize{
        // Fetch instruction
        let opcode = self.mem.read_8bit(self.pc);
        //console_print(format!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X}) LY: {} Cycles: {}", self.reg[A], self.reg[F], self.reg[B], self.reg[C], self.reg[D], self.reg[E], self.reg[H], self.reg[L],self.sp, self.pc, self.mem.read_8bit(self.pc), self.mem.read_8bit(self.pc+1),self.mem.read_8bit(self.pc+2),self.mem.read_8bit(self.pc+3),self.mem.read_8bit(0xFF44), self.total_cycles).as_str());
        if !self.unique_ops.contains(&opcode){
            self.unique_ops.push(opcode);
        }

        let cycle_count: usize = match opcode{
            // Special
            0x00 => {self.pc+=1; 4} // NOP
            0x27 => {self.daa(); self.pc+=1; 4} // DAA
            0x2F => {self.reg[A] = self.reg[A] ^ 0xFF; self.set_sub_flag(); self.set_half_carry_flag(); self.pc+=1; 4} //  CPL (Complement A)
            0x37 => {self.set_carry_flag(); self.clear_half_carry_flag(); self.clear_sub_flag(); self.pc+=1; 4} // SCF (Set Carry Flag)
            0x3F => {if self.is_carry_flag_set(){self.clear_carry_flag()}else{self.set_carry_flag()}; self.clear_half_carry_flag(); self.clear_sub_flag(); self.pc+=1; 4} // CCF (Flip Carry Flag)


            // 8 Bit register loads
            0x06 => {self.reg[B] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD B, d8
            0x16 => {self.reg[D] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD D, d8
            0x26 => {self.reg[H] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD H, d8
            0x36 => {self.mem.write_8bit(self.get_hl() as usize, self.mem.read_8bit(self.pc+1)); self.pc+=2; 12} // LD (HL), d8
            0x0E => {self.reg[C] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD C, d8
            0x1E => {self.reg[E] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD E, d8
            0x2E => {self.reg[L] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD L, d8
            0x3E => {self.reg[A] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD A, d8
            0x0A => {self.reg[A] = self.mem.read_8bit(self.get_bc() as usize); self.pc+=1; 8} // LD A, (BC)
            0x1A => {self.reg[A] = self.mem.read_8bit(self.get_de() as usize); self.pc+=1; 8} // LD A, (DE)
            0x2A => {self.reg[A] = self.mem.read_8bit(self.get_hl() as usize); self.inc_hl(); self.pc+=1; 8} // LD A, (HL+)
            0x3A => {self.reg[A] = self.mem.read_8bit(self.get_hl() as usize); self.dec_hl(); self.pc+=1; 8} // LD A, (HL-)
            0xF0 => {self.reg[A] = self.mem.read_8bit(self.mem.read_8bit(self.pc + 1) as usize + 0xFF00); self.pc+=2; 12} // LDH A, (a8)

            // 8 Bit register moves
            0x40 => {self.reg[B] = self.reg[B]; self.pc+=1; 4} // LD B, B
            0x41 => {self.reg[B] = self.reg[C]; self.pc+=1; 4} // LD B, C
            0x42 => {self.reg[B] = self.reg[D]; self.pc+=1; 4} // LD B, D
            0x43 => {self.reg[B] = self.reg[E]; self.pc+=1; 4} // LD B, E
            0x44 => {self.reg[B] = self.reg[H]; self.pc+=1; 4} // LD B, H
            0x45 => {self.reg[B] = self.reg[L]; self.pc+=1; 4} // LD B, L
            0x50 => {self.reg[D] = self.reg[B]; self.pc+=1; 4} // LD D, B
            0x51 => {self.reg[D] = self.reg[C]; self.pc+=1; 4} // LD D, C
            0x52 => {self.reg[D] = self.reg[D]; self.pc+=1; 4} // LD D, D
            0x53 => {self.reg[D] = self.reg[E]; self.pc+=1; 4} // LD D, E
            0x54 => {self.reg[D] = self.reg[H]; self.pc+=1; 4} // LD D, H
            0x55 => {self.reg[D] = self.reg[L]; self.pc+=1; 4} // LD D, L
            0x60 => {self.reg[H] = self.reg[B]; self.pc+=1; 4} // LD H, B
            0x61 => {self.reg[H] = self.reg[C]; self.pc+=1; 4} // LD H, C
            0x62 => {self.reg[H] = self.reg[D]; self.pc+=1; 4} // LD H, D
            0x63 => {self.reg[H] = self.reg[E]; self.pc+=1; 4} // LD H, E
            0x64 => {self.reg[H] = self.reg[H]; self.pc+=1; 4} // LD H, H
            0x65 => {self.reg[H] = self.reg[L]; self.pc+=1; 4} // LD H, L
            0x48 => {self.reg[C] = self.reg[B]; self.pc+=1; 4} // LD C, B
            0x49 => {self.reg[C] = self.reg[C]; self.pc+=1; 4} // LD C, C
            0x4A => {self.reg[C] = self.reg[D]; self.pc+=1; 4} // LD C, D
            0x4B => {self.reg[C] = self.reg[E]; self.pc+=1; 4} // LD C, E
            0x4C => {self.reg[C] = self.reg[H]; self.pc+=1; 4} // LD C, H
            0x4D => {self.reg[C] = self.reg[L]; self.pc+=1; 4} // LD C, L
            0x58 => {self.reg[E] = self.reg[B]; self.pc+=1; 4} // LD E, B
            0x59 => {self.reg[E] = self.reg[C]; self.pc+=1; 4} // LD E, C
            0x5A => {self.reg[E] = self.reg[D]; self.pc+=1; 4} // LD E, D
            0x5B => {self.reg[E] = self.reg[E]; self.pc+=1; 4} // LD E, E
            0x5C => {self.reg[E] = self.reg[H]; self.pc+=1; 4} // LD E, H
            0x5D => {self.reg[E] = self.reg[L]; self.pc+=1; 4} // LD E, L
            0x68 => {self.reg[L] = self.reg[B]; self.pc+=1; 4} // LD L, B
            0x69 => {self.reg[L] = self.reg[C]; self.pc+=1; 4} // LD L, C
            0x6A => {self.reg[L] = self.reg[D]; self.pc+=1; 4} // LD L, D
            0x6B => {self.reg[L] = self.reg[E]; self.pc+=1; 4} // LD L, E
            0x6C => {self.reg[L] = self.reg[H]; self.pc+=1; 4} // LD L, H
            0x6D => {self.reg[L] = self.reg[L]; self.pc+=1; 4} // LD L, L
            0x78 => {self.reg[A] = self.reg[B]; self.pc+=1; 4} // LD A, B
            0x79 => {self.reg[A] = self.reg[C]; self.pc+=1; 4} // LD A, C
            0x7A => {self.reg[A] = self.reg[D]; self.pc+=1; 4} // LD A, D
            0x7B => {self.reg[A] = self.reg[E]; self.pc+=1; 4} // LD A, E
            0x7C => {self.reg[A] = self.reg[H]; self.pc+=1; 4} // LD A, H
            0x7D => {self.reg[A] = self.reg[L]; self.pc+=1; 4} // LD A, L
            0x47 => {self.reg[B] = self.reg[A]; self.pc+=1; 4} // LD B, A
            0x57 => {self.reg[D] = self.reg[A]; self.pc+=1; 4} // LD D, A
            0x67 => {self.reg[H] = self.reg[A]; self.pc+=1; 4} // LD H, A
            0x4F => {self.reg[C] = self.reg[A]; self.pc+=1; 4} // LD C, A
            0x5F => {self.reg[E] = self.reg[A]; self.pc+=1; 4} // LD E, A
            0x6F => {self.reg[L] = self.reg[A]; self.pc+=1; 4} // LD L, A
            0x7F => {self.reg[A] = self.reg[A]; self.pc+=1; 4} // LD A, A
            0xFA => {let val = self.mem.read_16bit(self.pc + 1); self.reg[A] = self.mem.read_8bit(val as usize); self.pc+=3; 16} // LD A, (a16)

            // 8 Bit register stores
            0x70 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[B]); self.pc+=1; 8} // LD (HL), B
            0x71 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[C]); self.pc+=1; 8} // LD (HL), C
            0x72 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[D]); self.pc+=1; 8} // LD (HL), D
            0x73 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[E]); self.pc+=1; 8} // LD (HL), E
            0x74 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[H]); self.pc+=1; 8} // LD (HL), H
            0x75 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[L]); self.pc+=1; 8} // LD (HL), L
            0x77 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[A]); self.pc+=1; 8} // LD (HL), A
            0x46 => {self.reg[B] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD B, (HL)
            0x56 => {self.reg[D] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD D, (HL)
            0x66 => {self.reg[H] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD H, (HL)
            0x4E => {self.reg[C] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD C, (HL)
            0x5E => {self.reg[E] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD E, (HL)
            0x6E => {self.reg[L] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD L, (HL)
            0x7E => {self.reg[A] = self.mem.read_8bit(self.get_hl() as usize); self.pc+=1; 8} // LD A, (HL)
            0x02 => {self.mem.write_8bit(self.get_bc() as usize, self.reg[A]); self.pc+=1; 8} // LD (BC), A
            0x12 => {self.mem.write_8bit(self.get_de() as usize, self.reg[A]); self.pc+=1; 8} // LD (DE), A
            0x22 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[A]); self.inc_hl(); self.pc+=1; 8} // LD (HL+), A
            0x32 => {self.mem.write_8bit(self.get_hl() as usize, self.reg[A]); self.dec_hl(); self.pc+=1; 8} // LD (HL-), A
            0xE0 => {self.mem.write_8bit(self.mem.read_8bit(self.pc + 1) as usize + 0xFF00, self.reg[A]); self.pc+=2; 12} // LDH (a8), A

            // 16 Bit register loads
            0x01 => {let val = self.mem.read_16bit(self.pc + 1); self.set_bc(val); self.pc+=3; 12} // LD BC, d16
            0x11 => {let val = self.mem.read_16bit(self.pc + 1); self.set_de(val); self.pc+=3; 12} // LD DE, d16
            0x21 => {let val = self.mem.read_16bit(self.pc + 1); self.set_hl(val); self.pc+=3; 12} // LD HL, d16
            0x31 => {let val = self.mem.read_16bit(self.pc + 1); self.sp = val as usize; self.pc+=3; 12} // LD SP, d16
            0xF9 => {let val = self.get_hl(); self.sp = val as usize; self.pc+=1; 8} // LD SP, HL

            // 16 Bit register stores
            0xEA => {let val = self.mem.read_16bit(self.pc + 1); self.mem.write_8bit(val as usize, self.reg[A]); self.pc+=3; 16} // LD (a16), A
            0x08 => {let val = self.mem.read_16bit(self.pc + 1); self.mem.write_8bit(val as usize, (self.sp & 0xFF) as u8); self.mem.write_8bit((val+1) as usize, ((self.sp & 0xFF00)>>8) as u8);self.pc+=3; 20} // LD (a16), SP

            // 16 Bit register operations
            0x03 => {self.inc_bc(); self.pc+=1; 8} // INC BC
            0x13 => {self.inc_de(); self.pc+=1; 8} // INC DE
            0x23 => {self.inc_hl(); self.pc+=1; 8} // INC HL
            0x0B => {self.dec_bc(); self.pc+=1; 8} // DEC BC
            0x1B => {self.dec_de(); self.pc+=1; 8} // DEC DE
            0x2B => {self.dec_hl(); self.pc+=1; 8} // DEC HL
            0x33 => {self.sp = self.sp.wrapping_add(1); self.pc+=1; 8} // INC SP
            0x3B => {self.sp = self.sp.wrapping_sub(1)/* TO DO: self.sp -= 1 Underflow detection*/; self.pc+=1; 8} // DEC SP
            0x09 => {self.add_to_hl_and_set_flags(self.get_bc()); self.pc+=1; 8} // ADD HL, BC
            0x19 => {self.add_to_hl_and_set_flags(self.get_de()); self.pc+=1; 8} // ADD HL, DE
            0x29 => {self.add_to_hl_and_set_flags(self.get_hl()); self.pc+=1; 8} // ADD HL, HL
            0x39 => {self.add_to_hl_and_set_flags(self.sp as u16); self.pc+=1; 8} // ADD HL, SP

            // 8 Bit register operations
            0x04 => {self.reg[B] = self.reg[B].wrapping_add(1); self.clear_sub_flag(); if self.reg[B] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[B] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // INC B
            0x14 => {self.reg[D] = self.reg[D].wrapping_add(1); self.clear_sub_flag(); if self.reg[D] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[D] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // INC D
            0x24 => {self.reg[H] = self.reg[H].wrapping_add(1); self.clear_sub_flag(); if self.reg[H] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[H] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // INC H
            0x34 => {let mut value = self.mem.read_8bit(self.get_hl() as usize); value = value.wrapping_add(1); self.clear_sub_flag(); if value == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if value & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; self.mem.write_8bit(self.get_hl() as usize, value); 12} // INC (HL)
            0x0C => {self.reg[C] = self.reg[C].wrapping_add(1); self.clear_sub_flag(); if self.reg[C] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[C] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // INC C
            0x1C => {self.reg[E] = self.reg[E].wrapping_add(1); self.clear_sub_flag(); if self.reg[E] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[E] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // INC E
            0x2C => {self.reg[L] = self.reg[L].wrapping_add(1); self.clear_sub_flag(); if self.reg[L] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[L] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // LNC L
            0x3C => {self.reg[A] = self.reg[A].wrapping_add(1); self.clear_sub_flag(); if self.reg[A] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[A] & 0xF == 0x0{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // LNC A
            0x05 => {self.reg[B] = self.reg[B].wrapping_sub(1); self.set_sub_flag(); if self.reg[B] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[B] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC B
            0x15 => {self.reg[D] = self.reg[D].wrapping_sub(1); self.set_sub_flag(); if self.reg[D] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[D] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC D
            0x25 => {self.reg[H] = self.reg[H].wrapping_sub(1); self.set_sub_flag(); if self.reg[H] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[H] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC H
            0x35 => {let mut value = self.mem.read_8bit(self.get_hl() as usize); value = value.wrapping_sub(1); self.set_sub_flag(); if value == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if value & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; self.mem.write_8bit(self.get_hl() as usize, value); 12} // DEC (HL)
            0x0D => {self.reg[C] = self.reg[C].wrapping_sub(1); self.set_sub_flag(); if self.reg[C] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[C] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC C
            0x1D => {self.reg[E] = self.reg[E].wrapping_sub(1); self.set_sub_flag(); if self.reg[E] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[E] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC E
            0x2D => {self.reg[L] = self.reg[L].wrapping_sub(1); self.set_sub_flag(); if self.reg[L] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[L] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC L
            0x3D => {self.reg[A] = self.reg[A].wrapping_sub(1); self.set_sub_flag(); if self.reg[A] == 0{self.set_zero_flag();}else{self.clear_zero_flag();}; self.pc+=1; if self.reg[A] & 0xF == 0xF{self.set_half_carry_flag()}else{self.clear_half_carry_flag()}; 4} // DEC A

            // Jumps
            0xC3 => {self.pc = self.mem.read_16bit(self.pc + 1) as usize; 16} // JUMP
            0x20 => {if !self.is_zero_flag_set(){self.pc = ((self.pc as i32) + (self.mem.read_8bit(self.pc + 1) as i8 + 2) as i32) as usize; 12} else {self.pc+=2; 8}} // JR NZ, r8
            0x30 => {if !self.is_carry_flag_set(){self.pc = ((self.pc as i32) + (self.mem.read_8bit(self.pc + 1) as i8 + 2) as i32) as usize; 12} else {self.pc+=2; 8}} // JR NC, r8
            0x28 => {if self.is_zero_flag_set(){self.pc = ((self.pc as i32) + (self.mem.read_8bit(self.pc + 1) as i8 + 2) as i32) as usize; 12} else {self.pc+=2; 8}} // JR Z, r8
            0x38 => {if self.is_carry_flag_set(){self.pc = ((self.pc as i32) + (self.mem.read_8bit(self.pc + 1) as i8 + 2) as i32) as usize; 12} else {self.pc+=2; 8}} // JR C, r8
            0x18 => {self.pc = ((self.pc as i32) + (self.mem.read_8bit(self.pc + 1) as i8 + 2) as i32) as usize; 12} // JR r8
            0xE9 => {self.pc = self.get_hl() as usize; 4} // JP HL
            0xC2 => {if !self.is_zero_flag_set() {self.pc = self.mem.read_16bit(self.pc + 1) as usize; 16}else{self.pc+=3; 12}} // JUMP NZ
            0xD2 => {if !self.is_carry_flag_set() {self.pc = self.mem.read_16bit(self.pc + 1) as usize; 16}else{self.pc+=3; 12}} // JUMP NC

            // Calls
            0xCD => {self.mem.write_16bit(self.sp-1, self.pc as u16 + 3); self.sp-=2; self.pc = self.mem.read_16bit(self.pc + 1) as usize; 24} // CALL a16
            0xC4 => {if !self.is_zero_flag_set() {self.mem.write_16bit(self.sp-1, self.pc as u16 + 3); self.sp-=2; self.pc = self.mem.read_16bit(self.pc + 1) as usize; 24}else{self.pc+=3; 12}} // CALL NZ, a16
            0xD4 => {if !self.is_carry_flag_set() {self.mem.write_16bit(self.sp-1, self.pc as u16 + 3); self.sp-=2; self.pc = self.mem.read_16bit(self.pc + 1) as usize; 24}else{self.pc+=3; 12}} // CALL NC, a16
            0xCC => {if self.is_zero_flag_set() {self.mem.write_16bit(self.sp-1, self.pc as u16 + 3); self.sp-=2; self.pc = self.mem.read_16bit(self.pc + 1) as usize; 24}else{self.pc+=3; 12}} // CALL Z, a16
            0xDC => {if self.is_carry_flag_set() {self.mem.write_16bit(self.sp-1, self.pc as u16 + 3); self.sp-=2; self.pc = self.mem.read_16bit(self.pc + 1) as usize; 24}else{self.pc+=3; 12}} // CALL C, a16

            // Returns
            0xC9 => {let val = self.mem.read_16bit(self.sp+1); self.sp+=2; self.pc = val as usize; 16} // RET
            0xC0 => {if !self.is_zero_flag_set(){let val = self.mem.read_16bit(self.sp+1); self.sp+=2; self.pc = val as usize; 20}else{self.pc+=1; 8}} // RET NZ
            0xD0 => {if !self.is_carry_flag_set(){let val = self.mem.read_16bit(self.sp+1); self.sp+=2; self.pc = val as usize; 20}else{self.pc+=1; 8}} // RET NC
            0xC8 => {if self.is_zero_flag_set(){let val = self.mem.read_16bit(self.sp+1); self.sp+=2; self.pc = val as usize; 20}else{self.pc+=1; 8}} // RET Z
            0xD8 => {if self.is_carry_flag_set(){let val = self.mem.read_16bit(self.sp+1); self.sp+=2; self.pc = val as usize; 20}else{self.pc+=1; 8}} // RET C

            // Stack operations
            0xC5 => {self.mem.write_16bit(self.sp-1, self.get_bc()); self.sp-=2; self.pc+=1; 16} // PUSH BC
            0xD5 => {self.mem.write_16bit(self.sp-1, self.get_de()); self.sp-=2; self.pc+=1; 16} // PUSH DE
            0xE5 => {self.mem.write_16bit(self.sp-1, self.get_hl()); self.sp-=2; self.pc+=1; 16} // PUSH HL
            0xF5 => {self.mem.write_16bit(self.sp-1, self.get_af()); self.sp-=2; self.pc+=1; 16} // PUSH AF
            0xC1 => {let val = self.mem.read_16bit(self.sp+1); self.set_bc(val); self.sp+=2; self.pc+=1; 12} // POP BC
            0xD1 => {let val = self.mem.read_16bit(self.sp+1); self.set_de(val); self.sp+=2; self.pc+=1; 12} // POP DE
            0xE1 => {let val = self.mem.read_16bit(self.sp+1); self.set_hl(val); self.sp+=2; self.pc+=1; 12} // POP HL
            0xF1 => {let val = self.mem.read_16bit(self.sp+1); self.set_af(val); self.sp+=2; self.pc+=1; 12} // POP AF

            // ALU operations
            // Subtraction
            0x90 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[B]); self.pc+=1; 4} // SUB B
            0x91 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[C]); self.pc+=1; 4} // SUB C
            0x92 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[D]); self.pc+=1; 4} // SUB D
            0x93 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[E]); self.pc+=1; 4} // SUB E
            0x94 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[H]); self.pc+=1; 4} // SUB H
            0x95 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[L]); self.pc+=1; 4} // SUB L

            0x97 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.reg[A]); self.pc+=1; 4} // SUB A
            0x98 => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[B]); self.pc+=1; 4} // SUBC B
            0x99 => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[C]); self.pc+=1; 4} // SUBC C
            0x9A => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[D]); self.pc+=1; 4} // SUBC D
            0x9B => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[E]); self.pc+=1; 4} // SUBC E
            0x9C => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[H]); self.pc+=1; 4} // SUBC H
            0x9D => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[L]); self.pc+=1; 4} // SUBC L

            0x9F => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.reg[A]); self.pc+=1; 4} // SUBC A
            0xD6 => {self.reg[A] = self.sub_and_set_flags(self.reg[A], self.mem.read_8bit(self.pc+1)); self.pc+=2; 8} // SUB d8
            0xDE => {self.reg[A] = self.sub_with_carry_and_set_flags(self.reg[A], self.mem.read_8bit(self.pc+1)); self.pc+=2; 8} // SUBC d8

            // Addition
            0x80 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[B]); self.pc+=1; 4} // ADD B
            0x81 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[C]); self.pc+=1; 4} // ADD C
            0x82 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[D]); self.pc+=1; 4} // ADD D
            0x83 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[E]); self.pc+=1; 4} // ADD E
            0x84 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[H]); self.pc+=1; 4} // ADD H
            0x85 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[L]); self.pc+=1; 4} // ADD L
            0x87 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.reg[A]); self.pc+=1; 4} // ADD A
            0xC6 => {self.reg[A] = self.add_and_set_flags(self.reg[A], self.mem.read_8bit(self.pc+1)); self.pc+=2; 8} // ADD d8
            0x88 => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[B]); self.pc+=1; 4} // ADC B
            0x89 => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[C]); self.pc+=1; 4} // ADC C
            0x8A => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[D]); self.pc+=1; 4} // ADC D
            0x8B => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[E]); self.pc+=1; 4} // ADC E
            0x8C => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[H]); self.pc+=1; 4} // ADC H
            0x8D => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[L]); self.pc+=1; 4} // ADC L
            0x8F => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.reg[A]); self.pc+=1; 4} // ADC A
            0xCE => {self.reg[A] = self.add_with_carry_and_set_flags(self.reg[A], self.mem.read_8bit(self.pc+1)); self.pc+=2; 8} // ADC d8

            // Compare
            0xB8 => {self.sub_and_set_flags(self.reg[A], self.reg[B]); self.pc+=1; 4} // CP B
            0xB9 => {self.sub_and_set_flags(self.reg[A], self.reg[C]); self.pc+=1; 4} // CP C
            0xBA => {self.sub_and_set_flags(self.reg[A], self.reg[D]); self.pc+=1; 4} // CP D
            0xBB => {self.sub_and_set_flags(self.reg[A], self.reg[E]); self.pc+=1; 4} // CP E
            0xBC => {self.sub_and_set_flags(self.reg[A], self.reg[H]); self.pc+=1; 4} // CP H
            0xBD => {self.sub_and_set_flags(self.reg[A], self.reg[L]); self.pc+=1; 4} // CP L
            0xBF => {self.sub_and_set_flags(self.reg[A], self.reg[A]); self.pc+=1; 4} // CP A
            0xFE => {self.sub_and_set_flags(self.reg[A], self.mem.read_8bit(self.pc+1)); self.pc+=2; 8} // CP d8

            // Logical operations
            0xB0 => {self.reg[A] = self.reg[A] | self.reg[B]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR B
            0xB1 => {self.reg[A] = self.reg[A] | self.reg[C]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR C
            0xB2 => {self.reg[A] = self.reg[A] | self.reg[D]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR D
            0xB3 => {self.reg[A] = self.reg[A] | self.reg[E]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR E
            0xB4 => {self.reg[A] = self.reg[A] | self.reg[H]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR H
            0xB5 => {self.reg[A] = self.reg[A] | self.reg[L]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR L
            0xB6 => {self.reg[A] = self.reg[A] | (self.mem.read_8bit(self.get_hl() as usize)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 8} // OR (HL)
            0xB7 => {self.reg[A] = self.reg[A] | self.reg[A]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // OR A
            0xF6 => {self.reg[A] = self.reg[A] | (self.mem.read_8bit(self.pc+1)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=2; 8} // OR d8
            0xA0 => {self.reg[A] = self.reg[A] & self.reg[B]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND B
            0xA1 => {self.reg[A] = self.reg[A] & self.reg[C]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND C
            0xA2 => {self.reg[A] = self.reg[A] & self.reg[D]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND D
            0xA3 => {self.reg[A] = self.reg[A] & self.reg[E]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND E
            0xA4 => {self.reg[A] = self.reg[A] & self.reg[H]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND H
            0xA5 => {self.reg[A] = self.reg[A] & self.reg[L]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND L
            0xA6 => {self.reg[A] = self.reg[A] & (self.mem.read_8bit(self.get_hl() as usize)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 8} // AND (HL)
            0xA7 => {self.reg[A] = self.reg[A] & self.reg[A]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // AND A
            0xE6 => {self.reg[A] = self.reg[A] & (self.mem.read_8bit(self.pc+1)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.set_half_carry_flag(); self.clear_carry_flag(); self.pc+=2; 8} // AND d8
            0xA8 => {self.reg[A] = self.reg[A] ^ self.reg[B]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR B
            0xA9 => {self.reg[A] = self.reg[A] ^ self.reg[C]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR C
            0xAA => {self.reg[A] = self.reg[A] ^ self.reg[D]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR D
            0xAB => {self.reg[A] = self.reg[A] ^ self.reg[E]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR E
            0xAC => {self.reg[A] = self.reg[A] ^ self.reg[H]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR H
            0xAD => {self.reg[A] = self.reg[A] ^ self.reg[L]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR L
            0xAE => {self.reg[A] = self.reg[A] ^ (self.mem.read_8bit(self.get_hl() as usize)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 8} // XOR (HL)
            0xAF => {self.reg[A] = self.reg[A] ^ self.reg[A]; if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=1; 4} // XOR A
            0xEE => {self.reg[A] = self.reg[A] ^ (self.mem.read_8bit(self.pc+1)); if self.reg[A] == 0{self.set_zero_flag()}else{self.clear_zero_flag()}; self.clear_sub_flag(); self.clear_half_carry_flag(); self.clear_carry_flag(); self.pc+=2; 8} // XOR d8
            0x07 => {self.reg[A] = self.rotate_left_circular(self.reg[A]); self.clear_zero_flag(); self.pc+=1; 4} // RLC A
            0x17 => {self.reg[A] = self.rotate_left(self.reg[A]); self.clear_zero_flag(); self.pc+=1; 4} // RL A
            0x0F => {self.reg[A] = self.rotate_right_circular(self.reg[A]); self.clear_zero_flag(); self.pc+=1; 4} // RRC A
            0x1F => {self.reg[A] = self.rotate_right(self.reg[A]); self.clear_zero_flag(); self.pc+=1; 4} // RR A

            // CB prefixed operations
            0xCB => {self.shift_ops()}

            // HALT
            0x76 => {4} // HALT
            _ => {self.pc+=1;
                console_print(format!("Opcode not implemented: {:#04X} at {:#06X}", opcode, self.pc-1).as_str());
                4}
        };

        for _ in 0..(cycle_count/2) {
            self.frame_done |= self.ppu.run_ppu_cycle(&mut self.mem);
        }
        self.total_cycles += cycle_count as u64;
        return cycle_count;
    }


}