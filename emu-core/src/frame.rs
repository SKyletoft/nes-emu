use crate::ppu::{NesColour, Ppu};

pub trait NesFramebuffer {
	fn render(&mut self, ppu: &Ppu, lines: &[(i16, i16); 240]);

	fn update_tile(
		&mut self,
		tile_data: impl Iterator<Item = Option<NesColour>>,
		x: usize,
		y: usize,
		x_offset: usize,
	);

	fn update_sprite(&mut self, sprite_data: impl Iterator<Item = Option<NesColour>>, idx: usize);
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

	fn update_sprite(&mut self, _: impl Iterator<Item = Option<NesColour>>, _: usize) {}
}
