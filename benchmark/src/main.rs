use emu_core::{frame::NesFramebuffer, ppu::NesColour};

struct MockFramebuffer;

impl NesFramebuffer for MockFramebuffer {
	fn set(&mut self, _: usize, _: usize, _: NesColour) {}

	fn swap(&mut self) {}
}

fn main() {
	let game = Box::new(game::MAPPER.clone());
	let mut system_state = emu_core::interpret::State::new(game, MockFramebuffer);

	while system_state.rest.ppu.frame < 10000 {
		while system_state.rest.ppu_runahead <= 341 {
			game::nes_game(&mut system_state);
		}
		system_state.catch_up_ppu();
	}
}
