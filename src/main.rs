mod cpu;
mod inst;
mod nes_file;
mod interpret;
mod evaluate_instruction;

fn main() {
	let x = unsafe { evaluate_instruction::test() };
	println!("{x}");
}
