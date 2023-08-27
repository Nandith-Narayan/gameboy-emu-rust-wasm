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

            _ => {4}
        };

        return 0;
    }


}