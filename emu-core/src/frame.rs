use crate::{mapper::Mapper, ppu::Ppu};

pub trait NesFramebuffer {
	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, lines: &[(i16, i16); 240]);
}

#[derive(Clone, Debug)]
pub struct NoFramebuffer;

impl NesFramebuffer for NoFramebuffer {
	fn render<M: Mapper>(&mut self, _: &M, _: &Ppu, _: &[(i16, i16); 240]) {}
}
