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

	pub palettes: Palettes,
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
			palettes: Palettes([Palette([NesColour::DarkGrey; 4]); 8]),
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

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Pod, Zeroable, Into)]
pub struct Oam([Sprite; 64]);

impl Default for Oam {
	fn default() -> Self {
		Self::zeroed()
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Into)]
pub struct Palettes([Palette; 8]);

impl Palettes {
	pub fn as_raw_bytes(&self) -> &[u8; 64] {
		const _: () = {
			assert!(size_of::<Palettes>() == 32);
			assert!(size_of::<Palette>() == 4);
			assert!(align_of::<Palette>() >= align_of::<u8>());
		};
		unsafe { std::mem::transmute(self) }
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Into)]
pub struct Palette([NesColour; 4]);

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Zeroable)]
pub enum NesColour {
	Black = 0x0F,
	DarkGrey = 0x00,
	AzureDark = 0x01,
	BlueDark = 0x02,
	VioletDark = 0x03,
	MagentaDark = 0x04,
	RoseDark = 0x05,
	RedDark = 0x06,
	OrangeDark = 0x07,
	YellowDark = 0x08,
	ChartreuseDark = 0x09,
	GreenDark = 0x0A,
	SpringDark = 0x0B,
	CyanDark = 0x0C,
	LightGrey = 0x10,
	AzureMed = 0x11,
	BlueMed = 0x12,
	VioletMed = 0x13,
	MagentaMed = 0x14,
	RoseMed = 0x15,
	RedMed = 0x16,
	OrangeMed = 0x17,
	YellowMed = 0x18,
	ChartreuseMed = 0x19,
	GreenMed = 0x1A,
	SpringMed = 0x1B,
	CyanMed = 0x1C,
	White = 0x20,
	AzureLight = 0x21,
	BlueLight = 0x22,
	VioletLight = 0x23,
	MagentaLight = 0x24,
	RoseLight = 0x25,
	RedLight = 0x26,
	OrangeLight = 0x27,
	YellowLight = 0x28,
	ChartreuseLight = 0x29,
	GreenLight = 0x2A,
	SpringLight = 0x2B,
	CyanLight = 0x2C,
	AzurePale = 0x31,
	BluePale = 0x32,
	VioletPale = 0x33,
	MagentaPale = 0x34,
	RosePale = 0x35,
	RedPale = 0x36,
	OrangePale = 0x37,
	YellowPale = 0x38,
	ChartreusePale = 0x39,
	GreenPale = 0x3A,
	SpringPale = 0x3B,
	CyanPale = 0x3C,
}

// These colours are entirely untrusted and probably just hallucinated.
impl From<NesColour> for Colour {
	fn from(c: NesColour) -> Self {
		use NesColour::*;
		match c {
			Black => Colour {
				red: 0,
				green: 0,
				blue: 0,
				alpha: 255,
			},
			DarkGrey => Colour {
				red: 84,
				green: 84,
				blue: 84,
				alpha: 255,
			},
			AzureDark => Colour {
				red: 0,
				green: 30,
				blue: 116,
				alpha: 255,
			},
			BlueDark => Colour {
				red: 8,
				green: 16,
				blue: 144,
				alpha: 255,
			},
			VioletDark => Colour {
				red: 48,
				green: 0,
				blue: 136,
				alpha: 255,
			},
			MagentaDark => Colour {
				red: 68,
				green: 0,
				blue: 100,
				alpha: 255,
			},
			RoseDark => Colour {
				red: 92,
				green: 0,
				blue: 48,
				alpha: 255,
			},
			RedDark => Colour {
				red: 84,
				green: 4,
				blue: 0,
				alpha: 255,
			},
			OrangeDark => Colour {
				red: 60,
				green: 24,
				blue: 0,
				alpha: 255,
			},
			YellowDark => Colour {
				red: 32,
				green: 42,
				blue: 0,
				alpha: 255,
			},
			ChartreuseDark => Colour {
				red: 8,
				green: 58,
				blue: 0,
				alpha: 255,
			},
			GreenDark => Colour {
				red: 0,
				green: 64,
				blue: 0,
				alpha: 255,
			},
			SpringDark => Colour {
				red: 0,
				green: 60,
				blue: 0,
				alpha: 255,
			},
			CyanDark => Colour {
				red: 0,
				green: 50,
				blue: 60,
				alpha: 255,
			},
			LightGrey => Colour {
				red: 152,
				green: 150,
				blue: 152,
				alpha: 255,
			},
			AzureMed => Colour {
				red: 8,
				green: 76,
				blue: 196,
				alpha: 255,
			},
			BlueMed => Colour {
				red: 48,
				green: 50,
				blue: 236,
				alpha: 255,
			},
			VioletMed => Colour {
				red: 92,
				green: 30,
				blue: 228,
				alpha: 255,
			},
			MagentaMed => Colour {
				red: 136,
				green: 20,
				blue: 176,
				alpha: 255,
			},
			RoseMed => Colour {
				red: 160,
				green: 20,
				blue: 100,
				alpha: 255,
			},
			RedMed => Colour {
				red: 152,
				green: 34,
				blue: 32,
				alpha: 255,
			},
			OrangeMed => Colour {
				red: 120,
				green: 60,
				blue: 0,
				alpha: 255,
			},
			YellowMed => Colour {
				red: 84,
				green: 90,
				blue: 0,
				alpha: 255,
			},
			ChartreuseMed => Colour {
				red: 40,
				green: 114,
				blue: 0,
				alpha: 255,
			},
			GreenMed => Colour {
				red: 8,
				green: 124,
				blue: 0,
				alpha: 255,
			},
			SpringMed => Colour {
				red: 0,
				green: 118,
				blue: 40,
				alpha: 255,
			},
			CyanMed => Colour {
				red: 0,
				green: 102,
				blue: 120,
				alpha: 255,
			},
			White => Colour {
				red: 236,
				green: 238,
				blue: 236,
				alpha: 255,
			},
			AzureLight => Colour {
				red: 76,
				green: 154,
				blue: 236,
				alpha: 255,
			},
			BlueLight => Colour {
				red: 120,
				green: 124,
				blue: 236,
				alpha: 255,
			},
			VioletLight => Colour {
				red: 176,
				green: 98,
				blue: 236,
				alpha: 255,
			},
			MagentaLight => Colour {
				red: 228,
				green: 84,
				blue: 236,
				alpha: 255,
			},
			RoseLight => Colour {
				red: 236,
				green: 88,
				blue: 180,
				alpha: 255,
			},
			RedLight => Colour {
				red: 236,
				green: 106,
				blue: 100,
				alpha: 255,
			},
			OrangeLight => Colour {
				red: 212,
				green: 136,
				blue: 32,
				alpha: 255,
			},
			YellowLight => Colour {
				red: 160,
				green: 170,
				blue: 0,
				alpha: 255,
			},
			ChartreuseLight => Colour {
				red: 116,
				green: 196,
				blue: 0,
				alpha: 255,
			},
			GreenLight => Colour {
				red: 76,
				green: 208,
				blue: 32,
				alpha: 255,
			},
			SpringLight => Colour {
				red: 56,
				green: 204,
				blue: 108,
				alpha: 255,
			},
			CyanLight => Colour {
				red: 56,
				green: 180,
				blue: 204,
				alpha: 255,
			},
			AzurePale => Colour {
				red: 236,
				green: 238,
				blue: 236,
				alpha: 255,
			},
			BluePale => Colour {
				red: 168,
				green: 204,
				blue: 236,
				alpha: 255,
			},
			VioletPale => Colour {
				red: 188,
				green: 188,
				blue: 236,
				alpha: 255,
			},
			MagentaPale => Colour {
				red: 212,
				green: 178,
				blue: 236,
				alpha: 255,
			},
			RosePale => Colour {
				red: 236,
				green: 174,
				blue: 236,
				alpha: 255,
			},
			RedPale => Colour {
				red: 236,
				green: 174,
				blue: 212,
				alpha: 255,
			},
			OrangePale => Colour {
				red: 236,
				green: 180,
				blue: 176,
				alpha: 255,
			},
			YellowPale => Colour {
				red: 228,
				green: 196,
				blue: 144,
				alpha: 255,
			},
			ChartreusePale => Colour {
				red: 204,
				green: 210,
				blue: 120,
				alpha: 255,
			},
			GreenPale => Colour {
				red: 180,
				green: 222,
				blue: 120,
				alpha: 255,
			},
			SpringPale => Colour {
				red: 168,
				green: 226,
				blue: 144,
				alpha: 255,
			},
			CyanPale => Colour {
				red: 152,
				green: 226,
				blue: 180,
				alpha: 255,
			},
		}
	}
}
