use crate::core::constants::{A, B, C, D, E, H, L};
use crate::core::cpu::CPU;

impl CPU{
    // Executes the next instruction and returns number of CPU cycles executed
    pub fn execute(&mut self) -> usize{
        // Fetch instruction
        let opcode = self.mem.read_8bit(self.pc);

        let cycle_count: usize = match opcode{
            0x00 => {self.pc+=1; 4} // NOP

            // 8 Bit register loads
            0x06 => {self.reg[B] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD B, d8
            0x16 => {self.reg[D] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD D, d8
            0x26 => {self.reg[H] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD H, d8
            0x36 => {self.mem.write_8bit(self.get_hl() as usize, self.mem.read_8bit(self.pc+1)); self.pc+=2; 12} // LD (HL), d8
            0x0E => {self.reg[C] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD C, d8
            0x1E => {self.reg[E] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD E, d8
            0x2E => {self.reg[L] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD L, d8
            0x3E => {self.reg[A] = self.mem.read_8bit(self.pc+1); self.pc+=2; 8} // LD A, d8

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
            _ => {4}
        };

        return 0;
    }


}