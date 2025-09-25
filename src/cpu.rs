use bitfields::bitfield;
use bytemuck::Zeroable;

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Zeroable)]
pub struct P {
	#[bits(1)]
	c: bool,
	#[bits(1)]
	z: bool,
	#[bits(1)]
	i: bool,
	#[bits(1)]
	d: bool,
	#[bits(1)]
	b: bool,
	#[bits(1)]
	_unused: bool,
	#[bits(1)]
	v: bool,
	#[bits(1)]
	n: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Zeroable)]
pub struct Cpu {
	pub a: u8,
	pub x: u8,
	pub y: u8,
	pub s: u8,
	pub p: P,
	pub pc: u16,
}
