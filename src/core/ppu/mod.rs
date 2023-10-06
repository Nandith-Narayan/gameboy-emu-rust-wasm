use crate::core::ppu::PPUMode::OAMScan;

pub struct PPU{
    ppu_mode: PPUMode,
}

pub fn init_ppu() -> PPU{
    return PPU{
        ppu_mode: OAMScan,

    };
}

pub enum PPUMode {
    OAMScan,
    Drawing,
    HBlank,
    VBlank
}