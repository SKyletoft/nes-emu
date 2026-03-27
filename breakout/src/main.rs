use citro2d::{
	Instance, Point, Size,
	pixel_type::Rgb565,
	render::{Colour, Target},
	shapes::RectangleSolid,
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

	let tex = {
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

		let data2: [[Rgb565; 8]; 8] = std::array::from_fn(|i| {
			std::array::from_fn(|j| rgb565((i * 255 / 8) as u8, (j * 255 / 8) as u8, 0))
		});

		tex.swizzle_and_update_tile(data2, 0, 0);
		tex.swizzle_and_update_tile(data2, 1, 1);
		tex.swizzle_and_update_tile(data2, 3, 3);
		tex.swizzle_and_update_tile(data2, 6, 3);
		tex.swizzle_and_update_tile(data2, 1, 7);
		tex.swizzle_and_update_tile(data2, 7, 7);

		std::rc::Rc::new(tex)
	};

	const SPRITE_SCALE: f32 = 3.0;
	const SPRITE_SIZE: f32 = 64.0 * SPRITE_SCALE; // 192x192

	let sprite_left = Sprite::from_shared_tex(tex.clone())
		.with_size((SPRITE_SIZE, SPRITE_SIZE))
		.with_centre((SPRITE_SIZE / 2., SPRITE_SIZE / 2.))
		.with_pos((100., 120.))
		.with_mirroring(&Mirroring::Normal);

	let mut sprite_right = Sprite::from_shared_tex(tex)
		.with_size((SPRITE_SIZE, SPRITE_SIZE))
		.with_centre((SPRITE_SIZE / 2., SPRITE_SIZE / 2.))
		.with_pos((300., 120.));

	let mut uv_offset_x = 0_i32;
	let mut uv_offset_y = 0_i32;
	let mut uv_width = 64_i32;
	let mut uv_height = 64_i32;
	let mut rotation = 0_i32;

	while apt.main_loop() {
		hid.scan_input();
		let keys_down = hid.keys_down();

		if keys_down.contains(KeyPad::SELECT) {
			break;
		}

		if keys_down.contains(KeyPad::DPAD_UP) {
			uv_offset_y -= 8;
		}
		if keys_down.contains(KeyPad::DPAD_DOWN) {
			uv_offset_y += 8;
		}
		if keys_down.contains(KeyPad::DPAD_LEFT) {
			uv_offset_x -= 8;
		}
		if keys_down.contains(KeyPad::DPAD_RIGHT) {
			uv_offset_x += 8;
		}

		if keys_down.contains(KeyPad::A) {
			uv_offset_x -= 0;
			uv_width += 8;
		}
		if keys_down.contains(KeyPad::B) {
			uv_offset_x += 0;
			uv_width -= 8;
		}

		if keys_down.contains(KeyPad::X) {
			uv_offset_y -= 0;
			uv_height += 8;
		}
		if keys_down.contains(KeyPad::Y) {
			uv_offset_y += 0;
			uv_height -= 8;
		}

		if keys_down.contains(KeyPad::L) {
			rotation -= 45;
		}
		if keys_down.contains(KeyPad::R) {
			rotation += 45;
		}

		let left = uv_offset_x as f32 / 64.0;
		let right = (uv_offset_x + uv_width) as f32 / 64.0;
		let top = 1.0 - (uv_offset_y as f32 / 64.0);
		let bottom = 1.0 - ((uv_offset_y + uv_height) as f32 / 64.0);

		let rotation_radians = (rotation as f32).to_radians();

		println!("-------------------------");
		println!(
			"UV window: offset=({uv_offset_x}, {uv_offset_y}), size=({uv_width}, {uv_height})"
		);
		println!("UV coords: left={left:.2}, right={right:.2}, top={top:.2}, bottom={bottom:.2}");
		println!("rotation: {rotation} deg");

		sprite_right.set_mirroring(&Mirroring::Custom {
			left,
			right,
			top,
			bottom,
		});
		sprite_right.set_angle(rotation_radians);

		let ref_sprite_pos = (100.0, 120.0);
		let outline_left = ref_sprite_pos.0 - SPRITE_SIZE / 2.0 + uv_offset_x as f32 * SPRITE_SCALE;
		let outline_top = ref_sprite_pos.1 - SPRITE_SIZE / 2.0 + uv_offset_y as f32 * SPRITE_SCALE;
		let outline_width = uv_width as f32 * SPRITE_SCALE;
		let outline_height = uv_height as f32 * SPRITE_SCALE;

		c2d.render_target(&mut target, |_, t| {
			t.clear(Colour::new(255, 255, 255));

			t.render_2d_shape(&sprite_left);
			t.render_2d_shape(&sprite_right);

			let red = Colour::new(255, 0, 0);
			let pink = Colour::new(255, 200, 200);

			let wide = Size {
				width: outline_width + 2.0,
				height: 1.0,
			};
			let tall = Size {
				width: 1.0,
				height: outline_height + 2.0,
			};

			t.render_2d_shape(&RectangleSolid {
				point: Point {
					x: outline_left - 1.0,
					y: outline_top - 1.0,
					z: 1.0,
				},
				size: wide,
				colour: red,
			});
			t.render_2d_shape(&RectangleSolid {
				point: Point {
					x: outline_left - 1.0,
					y: outline_top + outline_height,
					z: 1.0,
				},
				size: wide,
				colour: pink,
			});
			t.render_2d_shape(&RectangleSolid {
				point: Point {
					x: outline_left - 1.0,
					y: outline_top - 1.0,
					z: 1.0,
				},
				size: tall,
				colour: red,
			});
			t.render_2d_shape(&RectangleSolid {
				point: Point {
					x: outline_left + outline_width,
					y: outline_top - 1.0,
					z: 1.0,
				},
				size: tall,
				colour: pink,
			});
		});
	}
}
