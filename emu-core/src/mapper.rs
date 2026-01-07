use bitfields::bitfield;

use crate::ppu::Ppu;

pub trait Mapper {
	fn get_cpu(&self, adr: u16) -> Option<u8>;
	fn set_cpu(&mut self, adr: u16, val: u8) -> Option<()>;
	fn get_ppu(&self, adr: u16, ppu: &Ppu) -> Option<u8>;
	fn set_ppu(&mut self, adr: u16, ppu: &mut Ppu, val: u8) -> Option<()>;
	fn get_palette_index(&self, half: bool, tile: u8, y: u8, x: u8) -> u8;
}

#[bitfield(u16)]
pub struct PatternAddress {
	#[bits(3)]
	fine_y: u8,
	#[bits(1)]
	plane: bool,
	#[bits(8)]
	tile_idx: u8,
	#[bits(1)]
	half: bool,
	#[bits(3, default = 0u8)]
	__unused: u8,
}
