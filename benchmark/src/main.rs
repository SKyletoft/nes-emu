fn main() {
	let mut system_state = emu_core::interpret::State::new(game::MAPPER.clone());

	while system_state.rest.ppu.frame < 10000 {
		while system_state.rest.ppu_runahead <= 341 {
			game::nes_game(&mut system_state);
		}
		system_state.catch_up_ppu();
	}
}
