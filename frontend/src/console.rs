use ctru::prelude::*;
use emu_core::{controller::ControllerState, interpret::State, mapper::Mapper, perf_stats};

use crate::citro2d_framebuffer::Citro2DFramebuffer;

pub fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());
	println!(" FRAME   CPU   PPU   GPU  FPS  ACTUAL");

	let framebuffer = Citro2DFramebuffer::new(&gfx).unwrap();

	#[cfg(feature = "compiled-game")]
	let game = game::MAPPER.clone().with_framebuffer(framebuffer);

	#[cfg(not(feature = "compiled-game"))]
	let game = emu_core::nrom256::NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes"))
		.unwrap()
		.with_framebuffer(framebuffer);

	let mut system_state = State::new(game);

	let mut last_frame = 0;

	while apt.main_loop() {
		while last_frame == system_state.rest.ppu.frame {
			while system_state.rest.ppu_runahead <= 341 {
				#[cfg(feature = "compiled-game")]
				game::nes_game(&mut system_state);

				#[cfg(not(feature = "compiled-game"))]
				system_state.next();
			}

			system_state.catch_up_ppu();
		}
		last_frame = system_state.rest.ppu.frame;

		let frame_count = system_state.rest.ppu.frame;
		let stats = perf_stats::get_and_reset_frame_stats();
		println!("{frame_count:5}: {stats}");

		hid.scan_input();

		let mut c = ControllerState::new();
		c.set_a(hid.keys_held().contains(KeyPad::A));
		c.set_b(hid.keys_held().contains(KeyPad::B) || hid.keys_held().contains(KeyPad::X));
		c.set_start(hid.keys_held().contains(KeyPad::START));
		c.set_select(hid.keys_held().contains(KeyPad::SELECT));
		c.set_up(hid.keys_held().contains(KeyPad::DPAD_UP));
		c.set_down(hid.keys_held().contains(KeyPad::DPAD_DOWN));
		c.set_left(hid.keys_held().contains(KeyPad::DPAD_LEFT));
		c.set_right(hid.keys_held().contains(KeyPad::DPAD_RIGHT));
		*system_state.rest.controller1.state_mut() = c.into_bits();

		if hid.keys_down().contains(KeyPad::L) {
			system_state.rest.rom.framebuffer().hide_left =
				!system_state.rest.rom.framebuffer().hide_left;
		}
		if hid.keys_down().contains(KeyPad::R) {
			system_state.rest.rom.framebuffer().hide_right =
				!system_state.rest.rom.framebuffer().hide_right;
		}

		if hid.keys_down().contains(KeyPad::SELECT) {
			break;
		}
	}
}
