mod cpu;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;

fn main() {
	let x = unsafe { evaluate_instruction::test() };
	println!("{x}");
}
