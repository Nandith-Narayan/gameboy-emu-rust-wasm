use crate::console_print;
use crate::core::cpu::CPU;

impl CPU{
    pub fn handle_interrupts(&mut self){
        if self.mem.interrupt_enable !=0{
            if self.mem.interrupt_request !=0{
                /*Interrupt Handler Routine:
                * Push PC to the stack
                * Set PC to the corresponding interrupt handler address
                * Clear the corresponding bit of the interrupt request flag (0xFF0F)
                * Clear the interrupt master enable flag
                */
                // V Blank Interrupt
                if self.mem.interrupt_request & 0b1 !=0 && self.mem.interrupt_enable & 0b1 !=0{
                    console_print("Entered V Blank");
                    self.mem.write_16bit(self.sp-1, self.pc as u16 + 3);
                    self.sp-=2;
                    self.pc = 0x40;
                    self.mem.interrupt_request &= 0b11110;
                    self.interrupt_master_enable = false;
                // LCD Interrupt
                }else if self.mem.interrupt_request & 0b10 !=0 && self.mem.interrupt_enable & 0b10 !=0{
                    console_print("LCD STAT interrupt");
                    self.mem.write_16bit(self.sp-1, self.pc as u16 + 3);
                    self.sp-=2;
                    self.pc = 0x48;
                    self.mem.interrupt_request &= 0b11101;
                    self.interrupt_master_enable = false;
                // Timer Interrupt
                }else if self.mem.interrupt_request & 0b100 !=0 && self.mem.interrupt_enable & 0b100 !=0{
                    self.mem.write_16bit(self.sp-1, self.pc as u16 + 3);
                    self.sp-=2;
                    self.pc = 0x50;
                    self.mem.interrupt_request &= 0b11011;
                    self.interrupt_master_enable = false;
                // Serial Interrupt
                }else if self.mem.interrupt_request & 0b1000 !=0 && self.mem.interrupt_enable & 0b1000 !=0{
                    self.mem.write_16bit(self.sp-1, self.pc as u16 + 3);
                    self.sp-=2;
                    self.pc = 0x58;
                    self.mem.interrupt_request &= 0b10111;
                    self.interrupt_master_enable = false;
                // Joypad Interrupt
                }else if self.mem.interrupt_request & 0b10000 !=0 && self.mem.interrupt_enable & 0b10000 !=0{
                    self.mem.write_16bit(self.sp-1, self.pc as u16 + 3);
                    self.sp-=2;
                    self.pc = 0x60;
                    self.mem.interrupt_request &= 0b01111;
                    self.interrupt_master_enable = false;
                }


            }
        }
    }
}