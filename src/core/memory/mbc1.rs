use crate::core::memory::{BankingMode, Memory};
use crate::core::memory::BankingMode::{RAM, ROM};

impl Memory{
    pub fn read_8bit_mbc1(&self, address: usize) -> u8{
        return match address {
            // Base ROM Bank
            0x0000..=0x3FFF => {self.rom[address]},
            // ROM Banks
            0x4000..=0x7FFF => {self.rom[address-0x4000 + self.rom_bank*0x4000]},
            // RAM Banks
            0xA000..=0xBFFF => {self.rom[address-0xA000 + self.ram_bank*0x2000]},

            _ => {0}
        };
    }

    pub fn write_8bit_mbc1(&mut self, address: usize, value: u8){
        match address {
            // RAM Enable
            0000..=0x1FFF => {},
            // RAM Banks
            0xA000..=0xBFFF => {self.rom[address-0xA000 + self.ram_bank*0x2000] = value;},
            // Set ROM Bank lower bits
            0x2000..=0x3FFF =>{self.rom_bank |= (value & 0x01F) as usize;},
            // Set RAM Bank or ROM Bank upper bits
            0x4000..=0x5FFF =>{
                match self.bank_mode{
                    BankingMode::ROM => {self.rom_bank |= (value & 0x0E0) as usize;},
                    BankingMode::RAM => {self.ram_bank = value as usize;}
                };
            },
            // Set Banking Mode
            0x6000..=0x7FFF => {
                if value%2==0{
                    self.bank_mode = ROM;
                }else{
                    self.bank_mode = RAM;
                }
            },
            _ => {}
        };
    }
}