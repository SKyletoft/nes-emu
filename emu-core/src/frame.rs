use crate::ppu::NesColour;

pub trait NesFramebuffer {
	fn set(&mut self, y: usize, x: usize, col: NesColour);
	fn swap(&mut self);
}
