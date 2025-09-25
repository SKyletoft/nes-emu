use bitflags::bitflags;
use bytemuck::{Pod, Zeroable};
use derive_more::derive::Into;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Ppu {
	pub ctrl: Ctrl,
	pub mask: Mask,
	pub status: Status,
	pub oam_adr: OamAdr,
	pub oam_data: OamData,
	pub scroll: Scroll,
	pub adr: Adr,
	pub data: Data,

	pub scanline: u16,
	pub dot: u16,
	pub vram: Vram,
	pub oam: Oam,
}

impl Default for Ppu {
	fn default() -> Self {
		Self {
			ctrl: Default::default(),
			mask: Default::default(),
			status: Default::default(),
			oam_adr: Default::default(),
			oam_data: Default::default(),
			scroll: Default::default(),
			adr: Default::default(),
			data: Default::default(),
			scanline: Default::default(),
			dot: Default::default(),
			vram: [0; _],
			oam: Oam::zeroed(),
		}
	}
}

pub type Vram = [u8; 2048];

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Ctrl(u8);

bitflags! {
	impl Ctrl: u8 {
		const NAMETABLE0 = 1 << 0;
		const NAMETABLE1 = 1 << 1;
		const VRAM_INCREMENT = 1 << 2;
		const SPRITE_TABLE = 1 << 3;
		const BACKGROUND_TABLE = 1 << 4;
		const SPRITE_SIZE = 1 << 5;
		const MASTER_SLAVE = 1 << 6;
		const NMI_ENABLE = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Mask(u8);

bitflags! {
	impl Mask: u8 {
		const GREYSCALE = 1 << 0;
		const SHOW_BG_LEFT = 1 << 1;
		const SHOW_SPR_LEFT = 1 << 2;
		const SHOW_BG = 1 << 3;
		const SHOW_SPR = 1 << 4;
		const EMPHASISE_RED = 1 << 5;
		const EMPHASISE_GREEN = 1 << 6;
		const EMPHASISE_BLUE = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Into)]
pub struct Status(u8);

bitflags! {
	impl Status: u8 {
		const SPRITE_OVERFLOW = 1 << 5;
		const SPRITE0_HIT = 1 << 6;
		const VBLANK = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Into)]
pub struct OamAdr(u8);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Into)]
pub struct OamData(u8);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Scroll {
	pub x: u8,
	pub y: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Adr {
	high: u8,
	low: u8,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Into)]
pub struct Data(u8);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Pod, Zeroable)]
pub struct Sprite {
	y: u8,
	tile: u8,
	attr: u8,
	x: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Pod, Zeroable)]
pub struct Oam([Sprite; 64]);

impl Default for Oam {
	fn default() -> Self {
		Self::zeroed()
	}
}
