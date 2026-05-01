use ctru::prelude::*;
use emu_core::{controller::ControllerState, interpret::State, mapper::Mapper, perf_stats};

use crate::{
	citro2d_framebuffer::Citro2DFramebuffer,
	debug_mode::{BackgroundView, DebugMode},
};

pub fn main() {
	let apt = Apt::new().unwrap();
	let mut hid = Hid::new().unwrap();
	let gfx = Gfx::new().unwrap();
	let _console = Console::new(gfx.bottom_screen.borrow_mut());
	// Abuses a bug in the console renderer that leaves the first line at the top regardless of scroll.
	println!(" FRAME   CPU  PPU  APU  GPU FPS ACTUAL");

	let framebuffer = Citro2DFramebuffer::new(&gfx).unwrap();

	#[cfg(feature = "compiled-game")]
	let game = game::MAPPER.clone().with_framebuffer(framebuffer);

	#[cfg(not(feature = "compiled-game"))]
	let game = emu_core::nrom::NROM256::parse_ines(include_bytes!("../../non-free/SMB1.nes"))
		.unwrap()
		.with_framebuffer(framebuffer);

	let mut system_state = State::new(game);

	let mut last_frame = 0;

	while apt.main_loop() {
		while last_frame == system_state.rest.ppu.frame {
			emu_core::perf_stats::start_cpu();
			while system_state.rest.ppu_runahead <= 341 {
				#[cfg(feature = "compiled-game")]
				game::nes_game(&mut system_state);

				#[cfg(not(feature = "compiled-game"))]
				system_state.next();
			}
			emu_core::perf_stats::stop_cpu();

			system_state.catch_up_ppu();
		}
		last_frame = system_state.rest.ppu.frame;

		let frame_count = system_state.rest.ppu.frame;
		let stats = perf_stats::get_and_reset_frame_stats();
		println!("{frame_count:5}: {stats}");

		hid.scan_input();
		let mut c = ControllerState::new();

		let fb = system_state.rest.rom.framebuffer();

		if hid.keys_down().contains(KeyPad::Y) {
			fb.debug_mode_enabled = !fb.debug_mode_enabled;
		}

		if !fb.debug_mode_enabled {
			c.set_a(hid.keys_held().contains(KeyPad::A));
			c.set_b(hid.keys_held().contains(KeyPad::B) || hid.keys_held().contains(KeyPad::X));
			c.set_start(hid.keys_held().contains(KeyPad::START));
			c.set_select(hid.keys_held().contains(KeyPad::SELECT));
			c.set_up(hid.keys_held().contains(KeyPad::DPAD_UP));
			c.set_down(hid.keys_held().contains(KeyPad::DPAD_DOWN));
			c.set_left(hid.keys_held().contains(KeyPad::DPAD_LEFT));
			c.set_right(hid.keys_held().contains(KeyPad::DPAD_RIGHT));
			if hid.keys_down().contains(KeyPad::L) {
				fb.hide_left = !fb.hide_left;
			}
			if hid.keys_down().contains(KeyPad::R) {
				fb.hide_right = !fb.hide_right;
			}
		} else {
			match fb.debug_mode {
				DebugMode::Backgrounds(view) => {
					if hid.keys_down().contains(KeyPad::DPAD_UP)
						|| hid.keys_down().contains(KeyPad::DPAD_DOWN)
					{
						fb.debug_mode = DebugMode::Sprites(0);
					}
					if hid.keys_down().contains(KeyPad::DPAD_LEFT) {
						fb.debug_mode = DebugMode::Backgrounds(view.prev());
					}
					if hid.keys_down().contains(KeyPad::DPAD_RIGHT) {
						fb.debug_mode = DebugMode::Backgrounds(view.next());
					}
					if hid.keys_down().contains(KeyPad::A) {
						fb.debug_background_mode = fb.debug_background_mode.next();
					}
				}
				DebugMode::Sprites(idx) => {
					if hid.keys_down().contains(KeyPad::DPAD_UP)
						|| hid.keys_down().contains(KeyPad::DPAD_DOWN)
					{
						fb.debug_mode = DebugMode::Backgrounds(BackgroundView::Both);
					}
					if hid.keys_down().contains(KeyPad::DPAD_LEFT) {
						fb.debug_mode = DebugMode::Sprites(if idx == 0 { 63 } else { idx - 1 });
					}
					if hid.keys_down().contains(KeyPad::DPAD_RIGHT) {
						fb.debug_mode = DebugMode::Sprites(if idx == 63 { 0 } else { idx + 1 });
					}
					if hid.keys_down().contains(KeyPad::A) {
						fb.debug_background_mode = fb.debug_background_mode.next();
					}
				}
			}
		}
		*system_state.rest.controller1.state_mut() = c.into_bits();

		if hid.keys_down().contains(KeyPad::SELECT) {
			break;
		}
	}
}
