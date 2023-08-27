pub struct Memory{
    rom: Vec<u8>, // Cartridge ROM
    ram: Vec<u8>, // Cartridge RAM
    vram: Vec<u8>, // Video RAM
    wram: Vec<u8>, // Work RAM
    oam: Vec<u8>, // Object Attribute Memory
    io_reg: Vec<u8>, // IO Registers
    hram: Vec<u8>, // High RAM
    rom_bank: usize,
    ram_bank: usize,


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
        rom_bank: 0,
        ram_bank: 0,
    };
}