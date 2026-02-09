use citro2d::{
	Instance,
	pixel_type::Rgba5551,
	render::Target,
	sprites::Sprite,
	texture::{ColourFormat, Tex},
};
use ctru::prelude::*;
use emu_core::{frame::NesFramebuffer, mapper::Mapper, ppu::Ppu};

use crate::colour::nes_colour_to_rgba5551;

pub struct Citro2DFramebuffer<'a> {
	instance: Instance,
	target: Target<'a>,

	bg1: Sprite,
	bg2: Sprite,
	sprites: [Sprite; 64],
}

impl<'a> Citro2DFramebuffer<'a> {
	pub fn new(gfx: &'a Gfx) -> Result<Self, citro2d::error::Error> {
		let top_screen = gfx.top_screen.borrow_mut();
		let instance = Instance::new()?;
		let target = Target::new(top_screen)?;

		let bg1 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));
		let bg2 = Sprite::from_tex(Tex::new(8 * 32, 8 * 32, ColourFormat::Rgba5551));

		let sprites = std::array::from_fn(|_| Sprite::new());

		Ok(Self {
			instance,
			target,
			bg1,
			bg2,
			sprites,
		})
	}
}

impl NesFramebuffer for Citro2DFramebuffer<'_> {
	fn render<M: Mapper>(&mut self, m: &M, ppu: &Ppu, _lines: &[(i16, i16); 240]) {
		let mut background: [[Rgba5551; 256]; 256] = [[Rgba5551::TRANSPARENT; _]; _];

		for (y, line) in background.iter_mut().take(240).enumerate() {
			for (x, col) in line.iter_mut().enumerate() {
				let new_col = m.get_bg_pixel(x as i16, y as i16, ppu, &ppu.palettes);
				// dbg!(*col, new_col);
				*col = nes_colour_to_rgba5551(new_col);
			}
		}
		self.bg1
			.texture_mut()
			.unwrap()
			.swizzle_and_upload::<Rgba5551, 256, 256, { 256 * 256 }>(
				&background, // &unsafe {std::mem::transmute::<_, [u8; size_of::<[[Rgba5551; 256]; 256]>()]>(background) }
			);

		for (y, line) in background.iter_mut().take(240).enumerate() {
			for (x, col) in line.iter_mut().enumerate() {
				*col =
					nes_colour_to_rgba5551(m.get_bg_pixel(x as i16, y as i16, ppu, &ppu.palettes));
			}
		}
		self.bg2
			.texture_mut()
			.unwrap()
			.swizzle_and_upload::<Rgba5551, 256, 256, { 256 * 256 }>(
				&background, // &unsafe {std::mem::transmute::<_, [u8; size_of::<[[Rgba5551; 256]; 256]>()]>(background) }
			);

		self.instance.render_target(&mut self.target, |_, t| {
			let emu_core::graphics::Colour {
				blue, green, red, ..
			} = emu_core::graphics::Colour::from_const(ppu.palettes[0][0]);
			t.clear(citro2d::render::Colour::new(red, green, blue));
			t.render_2d_shape(&self.bg1);
			t.render_2d_shape(&self.bg2);
		});
	}
}
