use emu_core::{evaluate_instruction::*, interpret::State, nrom256::NROM256, nrom128::NROM128};
use nesc::compile_nes_to_rust;

compile_nes_to_rust!("../non-free/SMB1.nes");
// compile_nes_to_rust!("../non-free/nestest.nes");
