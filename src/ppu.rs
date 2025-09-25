use bitfields::bitfield;
use bytemuck::{Pod, Zeroable};
use derive_more::derive::Into;

use crate::drawing::Colour;

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

impl Ppu {
	pub fn sprite_is_visible_x(&self, sprite: &Sprite) -> bool {
		self.dot <= sprite.x as u16 && (sprite.x as u16) < self.dot + self.sprite_width()
	}

	pub fn sprite_is_visible_y(&self, sprite: &Sprite) -> bool {
		self.scanline <= sprite.y as u16 && (sprite.y as u16) < self.scanline + 8
	}

	fn sprite_width(&self) -> u16 {
		if self.ctrl.sprite_size() { 16 } else { 8 }
	}

	pub fn sprite_get_colour(&self, sprite: &Sprite) -> Colour {
		todo!()
	}

	pub fn background_get_colour(&self) -> Colour {
		todo!()
	}
}

pub type Vram = [u8; 2048];

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ctrl {
	#[bits(1)]
	nametable_0: bool,
	#[bits(1)]
	nametable_1: bool,
	#[bits(1)]
	_unused: bool,
	#[bits(1)]
	vram_increment: bool,
	#[bits(1)]
	background_table: bool,
	#[bits(1)]
	sprite_size: bool,
	#[bits(1)]
	master_slave: bool,
	#[bits(1)]
	nmi_enable: bool,
}

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mask {
	#[bits(1)]
	greyscale: bool,
	#[bits(1)]
	show_bg_left: bool,
	#[bits(1)]
	show_spr_left: bool,
	#[bits(1)]
	show_bg: bool,
	#[bits(1)]
	show_spr: bool,
	#[bits(1)]
	emphasise_red: bool,
	#[bits(1)]
	emphasise_green: bool,
	#[bits(1)]
	emphasise_blue: bool,
}

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Status {
	#[bits(5)]
	_unused: u8,
	#[bits(1)]
	sprite_overflow: bool,
	#[bits(1)]
	sprite_0_hit: bool,
	#[bits(1)]
	vblank: bool,
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
	pub y: u8,
	pub tile: u8,
	pub attr: SpriteAttributes,
	pub x: u8,
}

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Pod, Zeroable)]
pub struct SpriteAttributes {
	#[bits(2)]
	palette: u8,
	#[bits(3)]
	_unused: u8,
	#[bits(1)]
	priority: bool,
	#[bits(1)]
	flip_h: bool,
	#[bits(1)]
	flip_v: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Pod, Zeroable, Into)]
pub struct Oam([Sprite; 64]);

impl Default for Oam {
	fn default() -> Self {
		Self::zeroed()
	}
}
