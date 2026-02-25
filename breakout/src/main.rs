#![feature(const_array, const_trait_impl)]

use citro2d::{
	Instance,
	pixel_type::Rgb565,
	render::{Colour, Target},
	sprites::{Mirroring, Sprite},
	texture::Tex,
};
use citro3d::texture::ColourFormat;
use ctru::prelude::*;

const TEXTURE_SIZE: usize = 64;

fn rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
	Rgb565::from_bits((((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3))
}

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	// let gfx = unsafe { Gfx::with_formats_vram(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8) } .unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut()); // Cannot exist if framebuffers are in vram
	println!();

	let mut c2d = Instance::new().unwrap();
	let mut target = Target::new(gfx.top_screen.borrow_mut()).unwrap();

	let mut sprite = {
		let mut tex = Tex::new(
			TEXTURE_SIZE as u16,
			TEXTURE_SIZE as u16,
			ColourFormat::Rgb565,
		);
		let data = std::array::from_fn(|i| {
			std::array::from_fn(|j| {
				rgb565(
					(i * 255 / TEXTURE_SIZE) as u8,
					(j * 255 / TEXTURE_SIZE) as u8,
					0,
				)
			})
		});
		tex.swizzle_and_upload::<Rgb565, TEXTURE_SIZE, TEXTURE_SIZE, { TEXTURE_SIZE * TEXTURE_SIZE }>(
			&data,
		);
		let mut sprite = Sprite::from_tex(tex);
		let (h, w) = sprite.size();
		sprite.set_centre((h / 2., w / 2.));
		sprite.set_pos((200., 120.));
		// sprite.set_mirroring(Mirroring::Normal);

		let data2: [[Rgb565; 8]; 8] = std::array::from_fn(|i| {
			std::array::from_fn(|j| rgb565((i * 255 / 8) as u8, (j * 255 / 8) as u8, 0))
		});

		sprite
			.texture_mut()
			.unwrap()
			.swizzle_and_update_tile(data2.clone(), 0, 0);
		sprite
			.texture_mut()
			.unwrap()
			.swizzle_and_update_tile(data2.clone(), 1, 1);
		sprite
			.texture_mut()
			.unwrap()
			.swizzle_and_update_tile(data2.clone(), 3, 3);

		sprite
	};

	let mut lr = false;
	let mut tb = false;

	while apt.main_loop() {
		hid.scan_input();
		let keys_down = hid.keys_down();

		if keys_down.contains(KeyPad::SELECT) {
			break;
		}

		if keys_down.contains(KeyPad::A) {
			lr = !lr;
		}
		if keys_down.contains(KeyPad::B) {
			tb = !tb;
		}

		let  (left, right) = if lr {
			(1., 0.)
		} else {
			(0., 1.)
		};
		let  (top, bottom) = if tb {
			(1., 0.)
		} else {
			(0., 1.)
		};
		let uv = Mirroring::Custom { left, top, right, bottom };
		sprite.set_mirroring(uv);

		println!("left: {left}, right: {right}, top: {top}, bottom: {bottom}");

		c2d.render_target(&mut target, |_i, t| {
			t.clear(Colour::new(255, 255, 255));
			t.render_2d_shape(&sprite);
		});
	}
}
