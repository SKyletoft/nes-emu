use crate::ppu::NesColour;

pub trait NesFramebuffer {
	fn set(&mut self, x: usize, y: usize, col: NesColour);
	fn swap(&mut self);
}
