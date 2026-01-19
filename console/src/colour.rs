use bitfields::bitfield;
use emu_core::{graphics::Colour, ppu::NesColour, unsafe_assert};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Bgr8 {
	blue: u8,
	green: u8,
	red: u8,
}

impl From<NesColour> for Bgr8 {
	fn from(c: NesColour) -> Self {
		use NesColour::*;
		match c {
			Black => Self {
				red: 0,
				green: 0,
				blue: 0,
			},
			DarkGrey => Self {
				red: 84,
				green: 84,
				blue: 84,
			},
			AzureDark => Self {
				red: 0,
				green: 30,
				blue: 116,
			},
			BlueDark => Self {
				red: 8,
				green: 16,
				blue: 144,
			},
			VioletDark => Self {
				red: 48,
				green: 0,
				blue: 136,
			},
			MagentaDark => Self {
				red: 68,
				green: 0,
				blue: 100,
			},
			RoseDark => Self {
				red: 92,
				green: 0,
				blue: 48,
			},
			RedDark => Self {
				red: 84,
				green: 4,
				blue: 0,
			},
			OrangeDark => Self {
				red: 60,
				green: 24,
				blue: 0,
			},
			YellowDark => Self {
				red: 32,
				green: 42,
				blue: 0,
			},
			ChartreuseDark => Self {
				red: 8,
				green: 58,
				blue: 0,
			},
			GreenDark => Self {
				red: 0,
				green: 64,
				blue: 0,
			},
			SpringDark => Self {
				red: 0,
				green: 60,
				blue: 0,
			},
			CyanDark => Self {
				red: 0,
				green: 50,
				blue: 60,
			},
			LightGrey => Self {
				red: 152,
				green: 150,
				blue: 152,
			},
			AzureMed => Self {
				red: 8,
				green: 76,
				blue: 196,
			},
			BlueMed => Self {
				red: 48,
				green: 50,
				blue: 236,
			},
			MagentaMed => Self {
				red: 136,
				green: 20,
				blue: 176,
			},
			RoseMed => Self {
				red: 160,
				green: 20,
				blue: 100,
			},
			RedMed => Self {
				red: 152,
				green: 34,
				blue: 32,
			},
			OrangeMed => Self {
				red: 120,
				green: 60,
				blue: 0,
			},
			YellowMed => Self {
				red: 84,
				green: 90,
				blue: 0,
			},
			ChartreuseMed => Self {
				red: 40,
				green: 114,
				blue: 0,
			},
			GreenMed => Self {
				red: 8,
				green: 124,
				blue: 0,
			},
			SpringMed => Self {
				red: 0,
				green: 118,
				blue: 40,
			},
			CyanMed => Self {
				red: 0,
				green: 102,
				blue: 120,
			},
			White => Self {
				red: 236,
				green: 238,
				blue: 236,
			},
			BlueLight => Self {
				red: 120,
				green: 124,
				blue: 236,
			},
			VioletLight => Self {
				red: 176,
				green: 98,
				blue: 236,
			},
			MagentaLight => Self {
				red: 228,
				green: 84,
				blue: 236,
			},
			RoseLight => Self {
				red: 236,
				green: 88,
				blue: 180,
			},
			RedLight => Self {
				red: 236,
				green: 106,
				blue: 100,
			},
			OrangeLight => Self {
				red: 212,
				green: 136,
				blue: 32,
			},
			YellowLight => Self {
				red: 160,
				green: 170,
				blue: 0,
			},
			ChartreuseLight => Self {
				red: 116,
				green: 196,
				blue: 0,
			},
			GreenLight => Self {
				red: 76,
				green: 208,
				blue: 32,
			},
			SpringLight => Self {
				red: 56,
				green: 204,
				blue: 108,
			},
			CyanLight => Self {
				red: 56,
				green: 180,
				blue: 204,
			},
			AzurePale => Self {
				red: 236,
				green: 238,
				blue: 236,
			},
			BluePale => Self {
				red: 168,
				green: 204,
				blue: 236,
			},
			VioletPale => Self {
				red: 188,
				green: 188,
				blue: 236,
			},
			MagentaPale => Self {
				red: 212,
				green: 178,
				blue: 236,
			},
			RosePale => Self {
				red: 236,
				green: 174,
				blue: 236,
			},
			RedPale => Self {
				red: 236,
				green: 174,
				blue: 212,
			},
			OrangePale => Self {
				red: 236,
				green: 180,
				blue: 176,
			},
			YellowPale => Self {
				red: 228,
				green: 196,
				blue: 144,
			},
			ChartreusePale => Self {
				red: 204,
				green: 210,
				blue: 120,
			},
			GreenPale => Self {
				red: 180,
				green: 222,
				blue: 120,
			},
			SpringPale => Self {
				red: 168,
				green: 226,
				blue: 144,
			},
			CyanPale => Self {
				red: 152,
				green: 226,
				blue: 180,
			},
			VioletMed => Self {
				red: 92,
				green: 30,
				blue: 228,
			},
			AzureLight => Self {
				red: 76,
				green: 154,
				blue: 236,
			},
		}
	}
}

#[derive(Copy, Clone)]
#[bitfield(u16, order = msb)]
pub struct Rgb565 {
	#[bits(5)]
	red: u8,
	#[bits(6)]
	green: u8,
	#[bits(5)]
	blue: u8,
}

impl From<Bgr8> for Rgb565 {
	fn from(bgr8: Bgr8) -> Self {
		let mut ret = Rgb565::new();
		ret.set_red(bgr8.red >> 3);
		ret.set_green(bgr8.green >> 2);
		ret.set_blue(bgr8.blue >> 3);
		ret
	}
}

impl From<NesColour> for Rgb565 {
	fn from(value: NesColour) -> Self {
		const fn convert_colour(c: NesColour) -> Rgb565 {
			let Colour {
				blue, green, red, ..
			} = Colour::from_const(c);
			let mut ret = Rgb565::new();
			ret.set_red(red >> 3);
			ret.set_green(green >> 2);
			ret.set_blue(blue >> 3);
			ret
		}
		const TRANSLATED_COLOURS: [Rgb565; 64] = NesColour::PALETTE.map(convert_colour);
		unsafe { unsafe_assert!((0..64).contains(&(value as usize))) };
		TRANSLATED_COLOURS[value as usize]
	}
}
