use emu_core::{evaluate_instruction_2::*, interpret::State, nrom256::NROM256};
use nesc_macro::compile_nes_to_rust;

compile_nes_to_rust!("../non-free/SMB1.nes");
