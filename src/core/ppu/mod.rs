mod sprite;

use std::collections::VecDeque;
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
    background_fifo: VecDeque<u8>,

    background_fetcher_mode: PPUBackgroundFetcherMode,
    scx: usize,
    scy: usize,
    ly: usize,
    lcdc: usize, // LCD control

    fetcher_x_pos: usize,
    tile_number: usize,
    tile_data_low: u8,
    tile_data_high: u8,
    lcd_x_pos: usize,
    pub frame_buffer: Vec<u8>,
}

pub fn init_ppu() -> PPU{
    return PPU{
        ppu_mode: OAMScan,
        cycle_count: 0,
        sprite_buffer: vec![],
        background_fifo: VecDeque::new(),

        background_fetcher_mode: FetchTileNumber,
        scx: 0,
        scy: 0,
        ly: 0,
        lcdc: 0,

        fetcher_x_pos: 0,
        tile_number: 0,
        tile_data_low: 0,
        tile_data_high: 0,
        lcd_x_pos: 0,
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

                self.fetcher_x_pos = 0;
                self.lcd_x_pos = 0;
                self.cycle_count += 2;
                // OAM scan lasts 80 T-cycles
                if self.cycle_count % CYCLES_PER_LINE >= 80{
                    self.ppu_mode = Drawing;
                }
            },
            // PPU actively drawing pixels state
            Drawing => {

                self.load_ppu_registers(mem);

                self.run_ppu_background_fetcher(mem);

                // Run pixel mixer
                if !self.background_fifo.is_empty(){
                    let color = self.background_fifo.pop_back().unwrap();
                    self.draw_pixel(self.lcd_x_pos, self.ly, color);
                    self.lcd_x_pos += 1;
                }

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
                    self.ly += 1;
                    mem.io_reg[0x44] = self.ly as u8;
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
                    self.ly = 0;
                    mem.io_reg[0x44] = self.ly as u8;
                    self.ppu_mode = OAMScan;
                    finished_frame = true;
                    self.cycle_count %= CYCLES_PER_FRAME;
                }else{
                    self.ly += 1;
                    mem.io_reg[0x44] = self.ly as u8;
                }
            },
        };

        return finished_frame;
    }

    fn run_ppu_background_fetcher(&mut self, mem: &mut Memory){
        match self.background_fetcher_mode{
            FetchTileNumber => {
                let mut tile_address: usize = 0x9800;
                if self.lcdc & 0x08 != 0{ // Bit 3 of LCDC selects background tile map (0=9800-9BFF, 1=9C00-9FFF)
                    tile_address = 0x9C00;
                }

                let x_offset= self.fetcher_x_pos + (self.scx/8) & 0x1F;
                let y_offset = 32 * (((self.ly + self.scy) & 0xFF) / 8);

                self.tile_number = mem.read_8bit(tile_address + ((x_offset + y_offset) & 0x3FF)) as usize;
                self.background_fetcher_mode = FetchTileDataLow;
            },
            FetchTileDataLow => {
                self.tile_data_low = mem.read_8bit(0x8000 +((self.tile_number * 16) + 2 * ((self.ly + self.scy) % 8)));
                self.background_fetcher_mode = FetchTileDataHigh;
            },
            FetchTileDataHigh => {
                self.tile_data_high = mem.read_8bit(0x8000 +(((self.tile_number * 16) + 2 * ((self.ly + self.scy) % 8)) + 1));
                self.background_fetcher_mode = PushToFIFO;
            },
            PushToFIFO => {
                if self.background_fifo.is_empty() {
                    for _ in 0..8{
                        let pixel = ((self.tile_data_high & 0x1) << 1) + (self.tile_data_low & 0x1);
                        self.tile_data_low >>= 1;
                        self.tile_data_high >>= 1;
                        self.background_fifo.push_front(pixel);
                    }
                    self.background_fetcher_mode = FetchTileNumber;
                    self.fetcher_x_pos += 1;
                }
            },
        }
    }

    fn draw_pixel(&mut self, x: usize, y: usize, color: u8){
        let base_address = (x + (y * 160)) * 3;
        let pixel = 4 - color;
        self.frame_buffer[base_address] = pixel * 60;
        self.frame_buffer[base_address+1] = pixel * 60;
        self.frame_buffer[base_address+2] = pixel * 60;
    }

    fn load_ppu_registers(&mut self, mem: &mut Memory){
        self.scx = mem.io_reg[0x43] as usize;
        self.scy = mem.io_reg[0x42] as usize;
        self.ly = mem.io_reg[0x44] as usize;
        self.lcdc = mem.io_reg[0x40] as usize;
    }

    // Function to render the contents of the current background map
    pub fn render_background_tile_data(&mut self, mem: &mut Memory) -> Vec<u8>{
        let mut debug_frame = vec![0u8; 256*256];
        for tile_y in 0..32{
            for tile_x in 0..32 {
                let tile_num = tile_y*32 + tile_x;
                for y in 0..8 {
                    let mut low_byte = mem.read_8bit(tile_num * 16 + 0x8000 + 2 + y*2);
                    let mut high_byte = mem.read_8bit(tile_num * 16 + 0x8000 + 1 + y*2);

                    for x in ((tile_x * 8)..(8 * tile_x + 8)).rev() {
                        let pixel = ((high_byte & 0x1) << 1) + (low_byte & 0x1);
                        low_byte >>= 1;
                        high_byte >>= 1;
                        debug_frame[(x) + (tile_y * 8 + y) * 256] = (4 - pixel) * 60;
                    }
                }
            }
        }
        return debug_frame;
    }
}

enum PPUBackgroundFetcherMode{
    FetchTileNumber,
    FetchTileDataLow,
    FetchTileDataHigh,
    PushToFIFO,
}

