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

fn clamp_uv(value: f32) -> f32 {
	match value {
		(..-0.1) => value + 0.1,
		(-0.1..0.0) => 0.,
		0.0..1.0 => value,
		1.0..1.1 => 1.,
		1.0.. => value - 0.1,
		_ => unreachable!(),
	}
}

fn clamp_angle(mut value: i32) -> f32 {
	value = value.rem_euclid(360);
	let angle = value % 45;
	let offset = value / 45;
	let clamped_angle = match angle {
		..5 => 0.,
		5..40 => (angle - 5) as f32 / 35. * 45.,
		40.. => 45.,
	};
	clamped_angle + offset as f32 * 45.
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

	let (mut left, mut top, mut right, mut bottom) = Mirroring::Normal.into();

	let mut rotation = 0;

	while apt.main_loop() {
		hid.scan_input();
		let keys_down = hid.keys_down();

		if keys_down.contains(KeyPad::SELECT) {
			break;
		}

		let keys_held = hid.keys_held();

		if keys_held.contains(KeyPad::A) {
			left -= 0.01;
			right += 0.01;
		}
		if keys_held.contains(KeyPad::B) {
			left += 0.01;
			right -= 0.01;
		}
		if keys_held.contains(KeyPad::X) {
			top -= 0.01;
			bottom += 0.01;
		}
		if keys_held.contains(KeyPad::Y) {
			top += 0.01;
			bottom -= 0.01;
		}

		if keys_held.contains(KeyPad::L) {
			rotation -= 1;
		}
		if keys_held.contains(KeyPad::R) {
			rotation += 1;
		}
		let rotation_clamped = clamp_angle(rotation);

		println!("-------------------------");
		println!("left: {left:0.2}, right: {right:0.2}, top: {top:0.2}, bottom: {bottom:0.2}");

		let left = clamp_uv(left);
		let right = clamp_uv(right);
		let top = clamp_uv(top);
		let bottom = clamp_uv(bottom);

		let uv = Mirroring::Custom {
			left,
			right,
			top,
			bottom,
		};
		sprite.set_mirroring(uv);
		sprite.set_angle(rotation_clamped.to_radians());

		println!("left: {left:0.2}, right: {right:0.2}, top: {top:0.2}, bottom: {bottom:0.2}");
		println!("rotation: {rotation_clamped:.2} rad");

		c2d.render_target(&mut target, |_, t| {
			t.clear(Colour::new(255, 255, 255));
			t.render_2d_shape(&sprite);
		});
	}
}
