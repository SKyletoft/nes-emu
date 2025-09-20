use bitflags::bitflags;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Ppu {
	pub ctrl: Ctrl,
	pub mask: Mask,
	pub status: Status,
	pub oam_addr: OamAddr,
	pub oam_data: OamData,
	pub scroll: Scroll,
	pub addr: Addr,
	pub data: Data,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Ctrl(u8);

bitflags! {
	impl Ctrl: u8 {
		const NAMETABLE0       = 1 << 0;
		const NAMETABLE1       = 1 << 1;
		const VRAM_INCREMENT   = 1 << 2;
		const SPRITE_TABLE     = 1 << 3;
		const BACKGROUND_TABLE = 1 << 4;
		const SPRITE_SIZE      = 1 << 5;
		const MASTER_SLAVE     = 1 << 6;
		const NMI_ENABLE       = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Mask(u8);

bitflags! {
	impl Mask: u8 {
		const GREYSCALE        = 1 << 0;
		const SHOW_BG_LEFT     = 1 << 1;
		const SHOW_SPR_LEFT    = 1 << 2;
		const SHOW_BG          = 1 << 3;
		const SHOW_SPR         = 1 << 4;
		const EMPHASISE_RED    = 1 << 5;
		const EMPHASISE_GREEN  = 1 << 6;
		const EMPHASISE_BLUE   = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Status(u8);

bitflags! {
	impl Status: u8 {
		const SPRITE_OVERFLOW = 1 << 5;
		const SPRITE0_HIT     = 1 << 6;
		const VBLANK          = 1 << 7;
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct OamAddr(u8);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct OamData(u8);

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Scroll {
	pub x: u8,
	pub y: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Addr {
	high: u8,
	low: u8,
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct Data(u8);
