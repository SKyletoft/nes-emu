use bitfields::bitfield;
use emu_core::ppu::NesColour;

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

impl From<NesColour> for Rgb565 {
	fn from(c: NesColour) -> Self {
		let mut res = Self::new();
		use NesColour::*;
		match c {
			Black => {
				res.set_red(0);
				res.set_blue(0);
				res.set_green(0);
			}
			DarkGrey => {
				res.set_red(84);
				res.set_green(84);
				res.set_blue(84);
			}
			AzureDark => {
				res.set_red(0);
				res.set_green(30);
				res.set_blue(116);
			}
			BlueDark => {
				res.set_red(8);
				res.set_green(16);
				res.set_blue(144);
			}
			VioletDark => {
				res.set_red(48);
				res.set_green(0);
				res.set_blue(136);
			}
			MagentaDark => {
				res.set_red(68);
				res.set_green(0);
				res.set_blue(100);
			}
			RoseDark => {
				res.set_red(92);
				res.set_green(0);
				res.set_blue(48);
			}
			RedDark => {
				res.set_red(84);
				res.set_green(4);
				res.set_blue(0);
			}
			OrangeDark => {
				res.set_red(60);
				res.set_green(24);
				res.set_blue(0);
			}
			YellowDark => {
				res.set_red(32);
				res.set_green(42);
				res.set_blue(0);
			}
			ChartreuseDark => {
				res.set_red(8);
				res.set_green(58);
				res.set_blue(0);
			}
			GreenDark => {
				res.set_red(0);
				res.set_green(64);
				res.set_blue(0);
			}
			SpringDark => {
				res.set_red(0);
				res.set_green(60);
				res.set_blue(0);
			}
			CyanDark => {
				res.set_red(0);
				res.set_green(50);
				res.set_blue(60);
			}
			LightGrey => {
				res.set_red(152);
				res.set_green(150);
				res.set_blue(152);
			}
			AzureMed => {
				res.set_red(8);
				res.set_green(76);
				res.set_blue(196);
			}
			BlueMed => {
				res.set_red(48);
				res.set_green(50);
				res.set_blue(236);
			}
			MagentaMed => {
				res.set_red(136);
				res.set_green(20);
				res.set_blue(176);
			}
			RoseMed => {
				res.set_red(160);
				res.set_green(20);
				res.set_blue(100);
			}
			RedMed => {
				res.set_red(152);
				res.set_green(34);
				res.set_blue(32);
			}
			OrangeMed => {
				res.set_red(120);
				res.set_green(60);
				res.set_blue(0);
			}
			YellowMed => {
				res.set_red(84);
				res.set_green(90);
				res.set_blue(0);
			}
			ChartreuseMed => {
				res.set_red(40);
				res.set_green(114);
				res.set_blue(0);
			}
			GreenMed => {
				res.set_red(8);
				res.set_green(124);
				res.set_blue(0);
			}
			SpringMed => {
				res.set_red(0);
				res.set_green(118);
				res.set_blue(40);
			}
			CyanMed => {
				res.set_red(0);
				res.set_green(102);
				res.set_blue(120);
			}
			White => {
				res.set_red(236);
				res.set_green(238);
				res.set_blue(236);
			}
			BlueLight => {
				res.set_red(120);
				res.set_green(124);
				res.set_blue(236);
			}
			VioletLight => {
				res.set_red(176);
				res.set_green(98);
				res.set_blue(236);
			}
			MagentaLight => {
				res.set_red(228);
				res.set_green(84);
				res.set_blue(236);
			}
			RoseLight => {
				res.set_red(236);
				res.set_green(88);
				res.set_blue(180);
			}
			RedLight => {
				res.set_red(236);
				res.set_green(106);
				res.set_blue(100);
			}
			OrangeLight => {
				res.set_red(212);
				res.set_green(136);
				res.set_blue(32);
			}
			YellowLight => {
				res.set_red(160);
				res.set_green(170);
				res.set_blue(0);
			}
			ChartreuseLight => {
				res.set_red(116);
				res.set_green(196);
				res.set_blue(0);
			}
			GreenLight => {
				res.set_red(76);
				res.set_green(208);
				res.set_blue(32);
			}
			SpringLight => {
				res.set_red(56);
				res.set_green(204);
				res.set_blue(108);
			}
			CyanLight => {
				res.set_red(56);
				res.set_green(180);
				res.set_blue(204);
			}
			AzurePale => {
				res.set_red(236);
				res.set_green(238);
				res.set_blue(236);
			}
			BluePale => {
				res.set_red(168);
				res.set_green(204);
				res.set_blue(236);
			}
			VioletPale => {
				res.set_red(188);
				res.set_green(188);
				res.set_blue(236);
			}
			MagentaPale => {
				res.set_red(212);
				res.set_green(178);
				res.set_blue(236);
			}
			RosePale => {
				res.set_red(236);
				res.set_green(174);
				res.set_blue(236);
			}
			RedPale => {
				res.set_red(236);
				res.set_green(174);
				res.set_blue(212);
			}
			OrangePale => {
				res.set_red(236);
				res.set_green(180);
				res.set_blue(176);
			}
			YellowPale => {
				res.set_red(228);
				res.set_green(196);
				res.set_blue(144);
			}
			ChartreusePale => {
				res.set_red(204);
				res.set_green(210);
				res.set_blue(120);
			}
			GreenPale => {
				res.set_red(180);
				res.set_green(222);
				res.set_blue(120);
			}
			SpringPale => {
				res.set_red(168);
				res.set_green(226);
				res.set_blue(144);
			}
			CyanPale => {
				res.set_red(152);
				res.set_green(226);
				res.set_blue(180);
			}
			VioletMed => {
				res.set_red(92);
				res.set_green(30);
				res.set_blue(228);
			}
			AzureLight => {
				res.set_red(76);
				res.set_green(154);
				res.set_blue(236);
			}
		}
		res
	}
}
