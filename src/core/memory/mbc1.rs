use crate::core::memory::Memory;

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
}