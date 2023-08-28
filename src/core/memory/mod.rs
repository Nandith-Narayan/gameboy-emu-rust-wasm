use js_sys::Atomics::add;
use crate::console_print;

mod mbc1;

pub struct Memory{
    pub rom: Vec<u8>, // Cartridge ROM
    ram: Vec<u8>, // Cartridge RAM
    vram: Vec<u8>, // Video RAM
    wram: Vec<u8>, // Work RAM
    oam: Vec<u8>, // Object Attribute Memory
    io_reg: Vec<u8>, // IO Registers
    hram: Vec<u8>, // High RAM
    rom_bank: usize,
    ram_bank: usize,
    bank_mode: BankingMode,

    pub debug_string: String,
}

pub fn init_memory() -> Memory{
    return Memory{
        rom: vec![0; 0x4000],
        ram: vec![0; 0x4000],
        vram: vec![0; 0x2000],
        wram: vec![0; 0x4000],
        oam: vec![0; 0x00A0],
        io_reg: vec![0; 0x0080],
        hram: vec![0; 0x0080],
        rom_bank: 1,
        ram_bank: 1,
        bank_mode: BankingMode::ROM,
        debug_string: "".to_string(),
    };
}

impl Memory {
    pub fn read_8bit(&self, address: usize) -> u8{
        return match address {
            // Common Memory Sections
            // Video RAM
            0x8000..=0x9FFF => {self.vram[address-0x8000]},
            // Work RAM
            0xC000..=0xCFFF => {self.wram[address-0xC000]},
            // Work RAM bank
            0xD000..=0xDFFF => {self.wram[address-0xC000]},
            // Object Attribute Memory
            0xFE00..=0xFE9F => {self.oam[address-0xFE00]},
            // IO Registers
            0xFF00..=0xFF7F => {self.io_reg[address-0xFF00]},
            // High RAM
            0xFF80..=0xFFFE => {self.hram[address-0xFF80]},

            // MBC mapped memory
            _ => {self.read_8bit_mbc1(address)}
        };
    }
    pub fn write_8bit(&mut self, address: usize, value: u8){
        match address {
            // Common Memory Sections
            // Video RAM
            0x8000..=0x9FFF => {self.vram[address-0x8000] = value;},
            // Work RAM
            0xC000..=0xCFFF => {self.wram[address-0xC000] = value;},
            // Work RAM bank
            0xD000..=0xDFFF => {self.wram[address-0xC000] = value;},
            // Object Attribute Memory
            0xFE00..=0xFE9F => {self.oam[address-0xFE00] = value;},
            // IO Registers
            0xFF00..=0xFF7F => {self.io_reg[address-0xFF00] = value;
                //console_print(format!("{:#06X} {:#04X}", address, value).as_str());
                if address == 0xFF02 && value == 0x81{
                    self.debug_string.push(self.io_reg[1] as char);
                }
            },
            // High RAM
            0xFF80..=0xFFFE => {self.hram[address-0xFF80] = value;},

            // MBC mapped memory
            _ => {self.write_8bit_mbc1(address, value);}
        };
    }
    pub fn read_16bit(&mut self, address: usize) -> u16{
        return ((self.read_8bit(address + 1) as u16) << 8)+(self.read_8bit(address) as u16);
    }
    pub fn write_16bit(&mut self, address: usize, value: u16){
        self.write_8bit(address, (value & 0xFF) as u8);
        self.write_8bit(address+1, ((value >> 8) & 0xFF) as u8);
    }
}

enum BankingMode{
    ROM,
    RAM,
}