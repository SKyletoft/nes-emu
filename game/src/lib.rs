use emu_core::{evaluate_instruction::*, interpret::State, nrom128::NROM128, nrom256::NROM256};
use nesc::compile_nes_to_rust;

compile_nes_to_rust!("../non-free/SMB1.nes");
// compile_nes_to_rust!("../non-free/nestest.nes");
