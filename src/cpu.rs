use bitflags::bitflags;

bitflags! {
	pub struct P: u8 {
		const C = 0b0000_0001;
		const Z = 0b0000_0010;
		const I = 0b0000_0100;
		const D = 0b0000_1000;
		const B = 0b0001_0000;
		const V = 0b0100_0000;
		const N = 0b1000_0000;

		const CZIDVNB
			= Self::C.bits()
			| Self::Z.bits()
			| Self::I.bits()
			| Self::D.bits()
			| Self::B.bits()
			| Self::V.bits()
			| Self::N.bits()
			| (1 << 4);
	}
}

#[repr(C)]
pub struct Cpu {
	a: u8,
	x: u8,
	y: u8,
	s: u8,
	p: P,

	pc: u16,
}
