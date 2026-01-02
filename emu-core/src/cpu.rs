use bitfields::bitfield;
use bytemuck::Zeroable;

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Zeroable)]
pub struct P {
	c: bool,
	z: bool,
	#[bits(default = true)]
	i: bool,
	d: bool,
	b: bool,
	#[bits(default = true, access = ro)]
	u: bool,
	v: bool,
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
