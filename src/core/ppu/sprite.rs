pub struct Sprite{
    y_pos: usize,
    x_pos: usize,
    tile_number: usize,
    obj_to_bg_priority_flag: bool,
    y_flip_flag: bool,
    x_flip_flag: bool,
    palette_number: bool,
}

pub fn create_sprite(byte_1: u8, byte_2: u8, byte_3: u8, byte_4: u8) -> Sprite{
    return Sprite{
        y_pos: byte_1 as usize,
        x_pos: byte_2 as usize,
        tile_number: byte_3 as usize,
        obj_to_bg_priority_flag: byte_4 & 0b10000000 != 0,
        y_flip_flag: byte_4 & 0b01000000 != 0,
        x_flip_flag: byte_4 & 0b00100000 != 0,
        palette_number: byte_4 & 0b00010000 != 0,
    }
}