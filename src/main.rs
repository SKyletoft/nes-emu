use cpu::P;
use interpret::State;
use nes_file::Mapper;

mod cpu;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;

fn display(state: &State) {
	let cpu::Cpu {a, x, y, s, p, pc, ..} = state.cpu;

	let c = p.c();
	let z = p.z();
	let i = p.i();
	let d = p.d();
	let b = p.b();
	let v = p.v();
	let n = p.n();

	let inst = state.next_inst();

	println!("┌─CPU──────────────────────────┐");
	println!("│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X} │");
	println!("│ C:{c} Z:{z} I:{i} D:{d} B:{b} V:{v} N:{n}  │");
	println!("└──────────────────────────────┘");
	println!("Next: {inst:?}");
}

fn main() {
	let path = std::env::args().nth(1).unwrap_or_else(|| "../non-free/SMB3.nes".into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = Mapper::try_from(buffer).unwrap();
	let mut system_state = State::new(game);

	let mut buf = String::new();
	loop {
		system_state = system_state.next_step();

		display(&system_state);
		buf.clear();
		std::io::stdin().read_line(&mut buf).unwrap();
	}
}
