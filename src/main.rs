use interpret::State;
use nes_file::NesFile;

mod cpu;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;

fn main() {
	let path = std::env::args().nth(1).unwrap_or_else(|| "../non-free/SMB3.nes".into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = NesFile::try_from(buffer).unwrap();
	let mut system_state = State::new(game);

	loop {
		system_state = system_state.interpret();
	}
}
