#![feature(const_array, const_trait_impl)]

use citro2d::{
	Instance,
	pixel_type::Rgb565,
	render::{Colour, Target},
	sprites::Sprite,
	texture::Tex,
};
use citro3d::texture::ColourFormat;
use ctru::{prelude::*, services::gspgpu::FramebufferFormat};

const NUM_TEXTURES: usize = 8 * 8 * 2; // = 2 cycles of 64-pixel swizzle blocks
const TEXTURE_SIZE: usize = 64;
const PIXELS_PER_TEXTURE: usize = TEXTURE_SIZE * TEXTURE_SIZE; // 256 pixels

// RGB565 format colours (little-endian bytes)
const BLACK: u16 = 0b00000_000000_00000;
const RED: u16 = 0b11111_000000_00000;
const BLUE: u16 = 0b00000_000000_11111;

fn create_texture_data(texture_index: usize) -> Vec<u8> {
	let red_pixel_pos = texture_index % PIXELS_PER_TEXTURE;
	let mut data = Vec::with_capacity(PIXELS_PER_TEXTURE * 2);
	for pixel_idx in 0..PIXELS_PER_TEXTURE {
		let colour = if pixel_idx == red_pixel_pos {
			RED
		} else if pixel_idx < red_pixel_pos {
			BLUE
		} else {
			BLACK
		};

		// Convert to little-endian bytes for RGB565
		data.push((colour & 0xFF) as u8); // Low byte
		data.push((colour >> 8) as u8); // High byte
	}

	data
}

fn rgb565(r: u8, g: u8, b: u8) -> Rgb565 {
	Rgb565::from_bits((((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3))
}

fn deadzone((dx, dy): (i16, i16)) -> (f32, f32) {
	let per_axis = |x: i16| {
		if x.abs() < 20 {
			0.0
		} else {
			(x as f32 / 130.).clamp(-1., 1.) * 5.
		}
	};
	(per_axis(dx), per_axis(dy))
}

fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = unsafe { Gfx::with_formats_vram(FramebufferFormat::Bgr8, FramebufferFormat::Bgr8) } .unwrap();
	// let gfx = Gfx::new().unwrap();
	// let _console = Console::new(gfx.bottom_screen.borrow_mut()); // Cannot exist if framebuffers are in vram

	let mut c2d = Instance::new().unwrap();
	let mut target = Target::new(gfx.top_screen.borrow_mut()).unwrap();

	let mut sprites: Vec<Sprite> = Vec::with_capacity(NUM_TEXTURES + 1);

	sprites.push({
		let mut tex = Tex::new(
			TEXTURE_SIZE as u16,
			TEXTURE_SIZE as u16,
			ColourFormat::Rgb565,
		);
		let data = std::array::from_fn(|i| {
			let i = 63 - i;
			std::array::from_fn(|j| {
				let j = 63 - j;
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

		let data2: [[Rgb565; 8]; 8] = unsafe {
			std::array::from_fn(|i| {
				std::array::from_fn(|j| rgb565((i * 255 / 8) as u8, (j * 255 / 8) as u8, 0))
				// std::array::from_fn(|j| rgb565(0, 0, 0))
			})
		};

		sprite.texture_mut().unwrap().update_tile(&data2, 0, 0);
		sprite.texture_mut().unwrap().update_tile(&data2, 1, 1);
		sprite.texture_mut().unwrap().update_tile(&data2, 3, 3);

		sprite
	});
	for i in 0..NUM_TEXTURES {
		let mut tex = Tex::new(
			TEXTURE_SIZE as u16,
			TEXTURE_SIZE as u16,
			ColourFormat::Rgb565,
		);
		let data = create_texture_data(i);
		tex.upload_swizzled(&data);
		let mut sprite = Sprite::from_tex(tex);
		let (h, w) = sprite.size();
		sprite.set_centre((h / 2., w / 2.));
		sprite.set_pos((200., 120.));
		sprites.push(sprite);
	}

	let mut current_texture: usize = 0;

	while apt.main_loop() {
		hid.scan_input();

		let (dx, dy) = deadzone(hid.circlepad_position());
		let (x, y) = sprites[current_texture].pos();

		sprites[current_texture].set_pos(((x + dx), (y - dy)));

		let mut angle = sprites[current_texture].angle();
		if hid.keys_held().contains(KeyPad::L) {
			angle -= 0.01;
		}
		if hid.keys_held().contains(KeyPad::R) {
			angle += 0.01;
		}
		sprites[current_texture].set_angle(angle);

		let (mut h, mut w) = sprites[current_texture].size();
		if hid.keys_held().contains(KeyPad::X) {
			h = (h + 1.).clamp(0.5, 240.);
			w = (w + 1.).clamp(0.5, 240.);
		}
		if hid.keys_held().contains(KeyPad::Y) {
			h = (h - 1.).clamp(0.5, 240.);
			w = (w - 1.).clamp(0.5, 240.);
		}
		sprites[current_texture].set_size((h, w));
		sprites[current_texture].set_centre((h / 2., w / 2.));

		let keys_down = hid.keys_down();
		if keys_down.contains(KeyPad::B) {
			if current_texture > 0 {
				current_texture -= 1;
			}
		}
		if keys_down.contains(KeyPad::A) {
			if current_texture < NUM_TEXTURES - 1 {
				current_texture += 1;
			}
		}

		if keys_down.contains(KeyPad::SELECT) {
			break;
		}

		c2d.render_target(&mut target, |_i, t| {
			t.clear(Colour::new(255, 255, 255));
			t.render_2d_shape(&sprites[current_texture]);
		});
	}
}
