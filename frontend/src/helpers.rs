use emu_core::ppu::{NesColour, Palette};

pub const TILE_SIZE: u32 = 8;
pub const BG_TILES: u32 = 32;
pub const BG_SIZE: u32 = TILE_SIZE * BG_TILES;
pub const PATTERN_TABLE_SIZE: u16 = 16 * 8;

#[rustfmt::skip]
pub const SWIZZLE_ORDER: [usize; 64] = [
	 0,  1,  8,  9,  2,  3, 10, 11,
	16, 17, 24, 25, 18, 19, 26, 27,
	 4,  5, 12, 13,  6,  7, 14, 15,
	20, 21, 28, 29, 22, 23, 30, 31,
	32, 33, 40, 41, 34, 35, 42, 43,
	48, 49, 56, 57, 50, 51, 58, 59,
	36, 37, 44, 45, 38, 39, 46, 47,
	52, 53, 60, 61, 54, 55, 62, 63,
];

#[derive(Copy, Clone)]
pub struct Sprite {
	pub palette: u8, /* is 0..4 */
	pub mirror_x: bool,
	pub mirror_y: bool,
	pub tile: u8,
}

pub const fn slice_palette([_, x, y, z]: Palette) -> [NesColour; 3] {
	[x, y, z]
}
