mod sprite;
use crate::core::memory::Memory;
use crate::core::ppu::PPUMode::*;
use crate::core::ppu::PPUBackgroundFetcherMode::*;
use crate::core::ppu::sprite::{create_sprite, Sprite};

pub const CYCLES_PER_FRAME: usize = 70224;
pub const CYCLES_PER_LINE: usize = 456;
pub const CYCLES_UNTIL_VBLANK: usize = 65664;

pub enum PPUMode {
    OAMScan,
    Drawing,
    HBlank,
    VBlank
}

pub struct PPU{
    pub ppu_mode: PPUMode,
    pub cycle_count: usize,
    sprite_buffer: Vec<Sprite>,

    background_fetcher_mode: PPUBackgroundFetcherMode,
    scx: usize,
    scy: usize,
    ly: usize,
    pub frame_buffer: Vec<u8>,
}

pub fn init_ppu() -> PPU{
    return PPU{
        ppu_mode: OAMScan,
        cycle_count: 0,
        sprite_buffer: vec![],

        background_fetcher_mode: FetchTileNumber,
        scx: 0,
        scy: 0,
        ly: 0,

        frame_buffer: vec![0; 160*144*3],
    };
}

impl PPU {
    pub fn run_ppu_cycle(&mut self, mem: &mut Memory) -> bool{
        let mut finished_frame = false;

        match &self.ppu_mode{
            // Scan Object Attribute Memory
            OAMScan => {
                // Fetch sprite
                let base = (self.cycle_count%CYCLES_PER_LINE)*2;
                let byte_1 = mem.oam[base];
                let byte_2 = mem.oam[base+1];
                let byte_3 = mem.oam[base+2];
                let byte_4 = mem.oam[base+3];
                let sprite = create_sprite(byte_1, byte_2, byte_3, byte_4);
                if self.sprite_buffer.len() < 10 {
                    self.sprite_buffer.push(sprite);
                }

                self.cycle_count += 2;
                // OAM scan lasts 80 T-cycles
                if self.cycle_count % CYCLES_PER_LINE >= 80{
                    self.ppu_mode = Drawing;
                }
            },
            // PPU actively drawing pixels state
            Drawing => {

                self.load_ppu_registers(mem);

                self.run_ppu_background_fetcher();

                self.cycle_count += 2;

                // Drawing Mode has a max duration of 289 T-Cycles
                // if this limit is exceeded, force the PPU to enter H-Blank mode.
                if self.cycle_count % CYCLES_PER_LINE >= 370{
                    self.ppu_mode = HBlank;
                }
            },
            // Horizontal Blanking Interval
            HBlank => {

                self.cycle_count += 2;

                // End scanline
                if self.cycle_count % CYCLES_PER_LINE == 0{
                    // If 144 lines worth of cycles have been completed, enter V-Blank mode
                    // else, enter the next line's OAM scan mode.
                    if self.cycle_count >= CYCLES_UNTIL_VBLANK{
                        self.ppu_mode = VBlank;
                    }else {
                        self.ppu_mode = OAMScan;
                    }
                }
            },
            // Vertical Blanking Interval
            VBlank => {

                self.cycle_count += 2;

                // Prepare to drawn the next frame
                if self.cycle_count >= CYCLES_PER_FRAME{
                    self.ppu_mode = OAMScan;
                    finished_frame = true;
                    self.cycle_count %= CYCLES_PER_FRAME;
                }
            },
        };

        return finished_frame;
    }

    fn run_ppu_background_fetcher(&mut self){
        match self.background_fetcher_mode{
            FetchTileNumber => {

            },
            FetchTileDataLow => {

            },
            FetchTileDataHigh => {

            },
            PushToFIFO => {

            },
        }
    }

    fn load_ppu_registers(&mut self, mem: &mut Memory){
        self.scx = mem.io_reg[0x43] as usize;
        self.scy = mem.io_reg[0x42] as usize;
        self.ly = mem.io_reg[0x44] as usize;
    }
}

enum PPUBackgroundFetcherMode{
    FetchTileNumber,
    FetchTileDataLow,
    FetchTileDataHigh,
    PushToFIFO,
}

