use crate::ppu::Ppu;

pub trait Mapper {
	fn get_cpu(&self, adr: u16) -> Option<u8>;
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()>;
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8>;
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()>;
}
