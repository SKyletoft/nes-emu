// #![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
// include!(concat!(env!("OUT_DIR"), "/cpubindings.rs"));

use bitflags::bitflags;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct P(u8);

bitflags! {
	impl P: u8 {
		const C = 1 << 0;
		const Z = 1 << 1;
		const I = 1 << 2;
		const D = 1 << 3;
		const B = 1 << 4;
		// unused bit 5
		const V = 1 << 6;
		const N = 1 << 7;
	}
}

impl P {
	pub fn new() -> Self {
		let mut p = P(0);
		p.0 |= 1 << 5;
		p
	}

	pub fn c (&self) -> u8 {
		self.contains(P::C) as u8
	}

	pub fn z (&self) -> u8 {
		self.contains(P::Z) as u8
	}

	pub fn i (&self) -> u8 {
		self.contains(P::I) as u8
	}

	pub fn d (&self) -> u8 {
		self.contains(P::D) as u8
	}

	pub fn b (&self) -> u8 {
		self.contains(P::B) as u8
	}

	pub fn v (&self) -> u8 {
		self.contains(P::V) as u8
	}

	pub fn n (&self) -> u8 {
		self.contains(P::N) as u8
	}
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Cpu {
	pub a: u8,
	pub x: u8,
	pub y: u8,
	pub s: u8,
	pub p: P,
	pub pc: u16,
}
