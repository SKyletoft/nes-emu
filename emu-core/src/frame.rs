use crate::ppu::{NesColour, Palette, Ppu};

pub trait NesFramebuffer {
	fn render(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]);

	fn update_tile(
		&mut self,
		tile_data: impl Iterator<Item = Option<NesColour>>,
		x: usize,
		y: usize,
		x_offset: usize,
	);

	fn update_sprite_pattern_table(
		&mut self,
		palette_idx: u8, /* is 0..4 */
		palette: Palette,
		tile_data: impl Iterator<Item = Option<NesColour>>,
	);

	fn update_sprite(
		&mut self,
		sprite_idx: usize,
		tile_idx: u8,
		horizontal: bool,
		vertical: bool,
		palette: u8, /* is 0..4 */
	);
}

#[derive(Clone, Debug)]
pub struct NoFramebuffer;

impl NesFramebuffer for NoFramebuffer {
	fn render(&mut self, _: &Ppu, _: &[(i16, i16); 240]) {}

	fn update_tile(
		&mut self,
		_: impl Iterator<Item = Option<NesColour>>,
		_: usize,
		_: usize,
		_: usize,
	) {
	}

	fn update_sprite_pattern_table(
		&mut self,
		_: u8, /* is 0..4 */
		_: Palette,
		_: impl Iterator<Item = Option<NesColour>>,
	) {
	}

	fn update_sprite(&mut self, _: usize, _: u8, _: bool, _: bool, _: u8 /* is 0..4 */) {}
}
