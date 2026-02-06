use crate::{mapper::Mapper, ppu::Ppu};

pub trait NesFramebuffer {
	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]);
}
