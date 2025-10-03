use std::fmt::{self, Write};

use crate::{cpu, drawing, inst::Inst, interpret::State, nes_file::Mapper};

fn print_instruction(state: &State, f: &mut String) -> fmt::Result {
	let instruction = state.next_inst();
	match instruction {
		Inst::AdcAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ADC ${:04X} = #${:02X}", adr, mem)
		}
		Inst::AdcAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ADC ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::AdcAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ADC ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::AdcImmediate(val) => write!(f, "ADC #${:02X}", val),
		Inst::AdcIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ADC (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::AdcIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ADC (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::AdcZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ADC ${:02X} = #${:02X}", adr, mem)
		}
		Inst::AdcZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ADC ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::AhxAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AHX ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::AhxIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "AHX (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::AlrImmediate(val) => write!(f, "ALR #${:02X}", val),
		Inst::AncImmediate2(val) => write!(f, "ANC #${:02X}", val),
		Inst::AncImmediate(val) => write!(f, "ANC #${:02X}", val),
		Inst::AndAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AND ${:04X} = #${:02X}", adr, mem)
		}
		Inst::AndAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AND ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::AndAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AND ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::AndImmediate(val) => write!(f, "AND #${:02X}", val),
		Inst::AndIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "AND (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::AndIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "AND (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::AndZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "AND ${:02X} = #${:02X}", adr, mem)
		}
		Inst::AndZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "AND ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::ArrImmediate(val) => write!(f, "ARR #${:02X}", val),
		Inst::AslAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ASL ${:04X} = #${:02X}", adr, mem)
		}
		Inst::AslAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ASL ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::AslAccumulator => write!(f, "ASL A"),
		Inst::AslZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ASL ${:02X} = #${:02X}", adr, mem)
		}
		Inst::AslZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ASL ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::AxsImmediate(val) => write!(f, "AXS #${:02X}", val),
		Inst::Bcc(offset) => write!(f, "BCC ${:02X}", offset),
		Inst::Bcs(offset) => write!(f, "BCS ${:02X}", offset),
		Inst::Beq(offset) => write!(f, "BEQ ${:02X}", offset),
		Inst::BitAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "BIT ${:04X} = #${:02X}", adr, mem)
		}
		Inst::BitZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "BIT ${:02X} = #${:02X}", adr, mem)
		}
		Inst::Bmi(offset) => write!(f, "BMI ${:02X}", offset),
		Inst::Bne(offset) => write!(f, "BNE ${:02X}", offset),
		Inst::Bpl(offset) => write!(
			f,
			"BPL ${:02X}",
			(offset as u16)
				.wrapping_add(state.cpu.pc)
				.wrapping_add(instruction.len() as u16)
		),
		Inst::Brk => write!(f, "BRK"),
		Inst::Bvc(offset) => write!(f, "BVC ${:02X}", offset),
		Inst::Bvs(offset) => write!(f, "BVS ${:02X}", offset),
		Inst::Clc => write!(f, "CLC"),
		Inst::Cld => write!(f, "CLD"),
		Inst::Cli => write!(f, "CLI"),
		Inst::Clv => write!(f, "CLV"),
		Inst::CmpAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CMP ${:04X} = #${:02X}", adr, mem)
		}
		Inst::CmpAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CMP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::CmpAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CMP ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::CmpImmediate(val) => write!(f, "CMP #${:02X}", val),
		Inst::CmpIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CMP (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::CmpIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CMP (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::CmpZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CMP ${:02X} = #${:02X}", adr, mem)
		}
		Inst::CmpZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CMP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::CpxAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CPX ${:04X} = #${:02X}", adr, mem)
		}
		Inst::CpxImmediate(val) => write!(f, "CPX #${:02X}", val),
		Inst::CpxZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CPX ${:02X} = #${:02X}", adr, mem)
		}
		Inst::CpyAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CPY ${:04X} = #${:02X}", adr, mem)
		}
		Inst::CpyImmediate(val) => write!(f, "CPY #${:02X}", val),
		Inst::CpyZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "CPY ${:02X} = #${:02X}", adr, mem)
		}
		Inst::DCPAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X} = #${:02X}", adr, mem)
		}
		Inst::DCPAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::DCPAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::DCPIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DCP (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::DCPIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DCP (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::DCPZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DCP ${:02X} = #${:02X}", adr, mem)
		}
		Inst::DCPZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DCP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::DecAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DEC ${:04X} = #${:02X}", adr, mem)
		}
		Inst::DecAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DEC ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::DecZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DEC ${:02X} = #${:02X}", adr, mem)
		}
		Inst::DecZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "DEC ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Dex => write!(f, "DEX"),
		Inst::Dey => write!(f, "DEY"),
		Inst::EorAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X} = #${:02X}", adr, mem)
		}
		Inst::EorAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::EorAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::EorImmediate(val) => write!(f, "EOR #${:02X}", val),
		Inst::EorIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "EOR (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::EorIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "EOR (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::EorZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "EOR ${:02X} = #${:02X}", adr, mem)
		}
		Inst::EorZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "EOR ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::IncAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "INC ${:04X} = #${:02X}", adr, mem)
		}
		Inst::IncAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "INC ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::IncZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "INC ${:02X} = #${:02X}", adr, mem)
		}
		Inst::IncZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "INC ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Inx => write!(f, "INX"),
		Inst::Iny => write!(f, "INY"),
		Inst::ISCAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X} = #${:02X}", adr, mem)
		}
		Inst::ISCAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::ISCAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::ISCIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ISC (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::ISCIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ISC (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::ISCZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ISC ${:02X} = #${:02X}", adr, mem)
		}
		Inst::ISCZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ISC ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::JmpAbsolute(adr) => write!(f, "JMP ${:04X}", adr),
		Inst::JmpIndirect(adr) => write!(f, "JMP (${:04X})", adr),
		Inst::Jsr(adr) => write!(f, "JSR ${:04X}", adr),
		Inst::LASAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAS ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::LAXAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAX ${:04X} = #${:02X}", adr, mem)
		}
		Inst::LAXAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAX ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::LAXImmediate(val) => write!(f, "LAX #${:02X}", val),
		Inst::LAXIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LAX (${:02X},X) = #${:02X}", adr, mem)
		}
		Inst::LAXIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LAX (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::LAXZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LAX ${:02X} = #${:02X}", adr, mem)
		}
		Inst::LAXZeroPageY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LAX ${:02X},Y = #${:02X}", adr, mem)
		}
		Inst::LdaAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDA ${:04X} = #${:02X}", adr, mem)
		}
		Inst::LdaAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDA ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::LdaAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDA ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::LdaImmediate(val) => write!(f, "LDA #${:02X}", val),
		Inst::LdaIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDA (${:02X},X) = #${:02X}", adr, mem)
		}
		Inst::LdaIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDA (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::LdaZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDA ${:02X} = #${:02X}", adr, mem)
		}
		Inst::LdaZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDA ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::LdxAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDX ${:04X} = #${:02X}", adr, mem)
		}
		Inst::LdxAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDX ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::LdxImmediate(val) => write!(f, "LDX #${:02X}", val),
		Inst::LdxZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDX ${:02X} = #${:02X}", adr, mem)
		}
		Inst::LdxZeroPageY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDX ${:02X},Y = #${:02X}", adr, mem)
		}
		Inst::LdyAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDY ${:04X} = #${:02X}", adr, mem)
		}
		Inst::LdyAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDY ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::LdyImmediate(val) => write!(f, "LDY #${:02X}", val),
		Inst::LdyZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDY ${:02X} = #${:02X}", adr, mem)
		}
		Inst::LdyZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LDY ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::LsrAccumulator => write!(f, "LSR A"),
		Inst::LsrAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LSR ${:04X} = #${:02X}", adr, mem)
		}
		Inst::LsrAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LSR ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::LsrZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LSR ${:02X} = #${:02X}", adr, mem)
		}
		Inst::LsrZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "LSR ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Nop2 => write!(f, "NOP"),
		Inst::NOP3 => write!(f, "NOP"),
		Inst::NOP4 => write!(f, "NOP"),
		Inst::NOP5 => write!(f, "NOP"),
		Inst::NOP6 => write!(f, "NOP"),
		Inst::NOP7 => write!(f, "NOP"),
		Inst::NOP8 => write!(f, "NOP"),
		Inst::NOP9 => write!(f, "NOP"),
		Inst::NOP10 => write!(f, "NOP"),
		Inst::NOP11 => write!(f, "NOP"),
		Inst::NOP12 => write!(f, "NOP"),
		Inst::NOP13 => write!(f, "NOP"),
		Inst::NOP14 => write!(f, "NOP"),
		Inst::NOP15 => write!(f, "NOP"),
		Inst::NOP16 => write!(f, "NOP"),
		Inst::NOP17 => write!(f, "NOP"),
		Inst::NOP18 => write!(f, "NOP"),
		Inst::NOP19 => write!(f, "NOP"),
		Inst::NOP20 => write!(f, "NOP"),
		Inst::NOP21 => write!(f, "NOP"),
		Inst::NOP22 => write!(f, "NOP"),
		Inst::NOPAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X} = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX2(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX3(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX4(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX5(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPAbsoluteX6(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::NOPImmediate(val) => write!(f, "NOP #${:02X}", val),
		Inst::NOPImmediate2(val) => write!(f, "NOP #${:02X}", val),
		Inst::NOPImmediate3(_val) => write!(f, "NOP"),
		Inst::NOPZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X} = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPage3(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X} = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPage4(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X} = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX2(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX3(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX4(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX5(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::NOPZeroPageX6(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "NOP ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::OraAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ORA ${:04X} = #${:02X}", adr, mem)
		}
		Inst::OraAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ORA ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::OraAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ORA ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::OraImmediate(val) => write!(f, "ORA #${:02X}", val),
		Inst::OraIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ORA (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::OraIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ORA (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::OraZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ORA ${:02X} = #${:02X}", adr, mem)
		}
		Inst::OraZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ORA ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Pha => write!(f, "PHA"),
		Inst::Php => write!(f, "PHP"),
		Inst::Pla => write!(f, "PLA"),
		Inst::Plp => write!(f, "PLP"),
		Inst::RLAAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X} = #${:02X}", adr, mem)
		}
		Inst::RLAAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::RLAAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::RLAIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RLA (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::RLAIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RLA (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::RLAZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RLA ${:02X} = #${:02X}", adr, mem)
		}
		Inst::RLAZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RLA ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::RolAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROL ${:04X} = #${:02X}", adr, mem)
		}
		Inst::RolAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROL ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::RolAccumulator => write!(f, "ROL A"),
		Inst::RolZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ROL ${:02X} = #${:02X}", adr, mem)
		}
		Inst::RolZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ROL ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::RorAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROR ${:04X} = #${:02X}", adr, mem)
		}
		Inst::RorAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROR ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::RorAccumulator => write!(f, "ROR A"),
		Inst::RorZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ROR ${:02X} = #${:02X}", adr, mem)
		}
		Inst::RorZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "ROR ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::RRAAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X} = #${:02X}", adr, mem)
		}
		Inst::RRAAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::RRAAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::RRAIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RRA (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::RRAIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RRA (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::RRAZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RRA ${:02X} = #${:02X}", adr, mem)
		}
		Inst::RRAZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "RRA ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Rti => write!(f, "RTI"),
		Inst::Rts => write!(f, "RTS"),
		Inst::SAXAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SAX ${:04X} = #${:02X}", adr, mem)
		}
		Inst::SAXIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SAX (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::SAXZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SAX ${:02X} = #${:02X}", adr, mem)
		}
		Inst::SAXZeroPageY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SAX ${:02X},Y = #${:02X}", adr, mem)
		}
		Inst::SbcAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SBC ${:04X} = #${:02X}", adr, mem)
		}
		Inst::SbcAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SBC ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::SbcAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SBC ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::SbcImmediate(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcImmediate2(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SBC (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::SbcIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SBC (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::SbcZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SBC ${:02X} = #${:02X}", adr, mem)
		}
		Inst::SbcZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SBC ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::Sec => write!(f, "SEC"),
		Inst::Sed => write!(f, "SED"),
		Inst::Sei => write!(f, "SEI"),
		Inst::SHXAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SHX ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::SHYAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SHY ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::SLOAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X} = #${:02X}", adr, mem)
		}
		Inst::SLOAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::SLOAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::SLOIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SLO (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::SLOIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SLO (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::SLOZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SLO ${:02X} = #${:02X}", adr, mem)
		}
		Inst::SLOZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SLO ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::SREAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X} = #${:02X}", adr, mem)
		}
		Inst::SREAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::SREAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::SREIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SRE (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::SREIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SRE (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::SREZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SRE ${:02X} = #${:02X}", adr, mem)
		}
		Inst::SREZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "SRE ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::StaAbsolute(x) => {
			let mem = state.mem_pure(x.into());
			write!(f, "STA ${:04X} = #${:02X}", x, mem)
		}
		Inst::StaAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STA ${:04X},X = #${:02X}", adr, mem)
		}
		Inst::StaAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STA ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::StaIndirectX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STA (${:02X}),X = #${:02X}", adr, mem)
		}
		Inst::StaIndirectY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STA (${:02X}),Y = #${:02X}", adr, mem)
		}
		Inst::StaZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STA ${:02X} = #${:02X}", adr, mem)
		}
		Inst::StaZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STA ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::StxAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STX ${:04X} = #${:02X}", adr, mem)
		}
		Inst::StxZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STX ${:02X} = #${:02X}", adr, mem)
		}
		Inst::StxZeroPageY(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STX ${:02X},Y = #${:02X}", adr, mem)
		}
		Inst::StyAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STY ${:04X} = #${:02X}", adr, mem)
		}
		Inst::StyZeroPage(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STY ${:02X} = #${:02X}", adr, mem)
		}
		Inst::StyZeroPageX(adr) => {
			let mem = state.mem_pure((adr as u16).into());
			write!(f, "STY ${:02X},X = #${:02X}", adr, mem)
		}
		Inst::TASAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "TAS ${:04X},Y = #${:02X}", adr, mem)
		}
		Inst::Tax => write!(f, "TAX"),
		Inst::Tay => write!(f, "TAY"),
		Inst::Tsx => write!(f, "TSX"),
		Inst::Txa => write!(f, "TXA"),
		Inst::Txs => write!(f, "TXS"),
		Inst::Tya => write!(f, "TYA"),
		Inst::XAAImmediate(val) => write!(f, "XAA #${:02X}", val),
	}
}

#[cfg(test)]
fn fceux_log(state: &State) -> String {
	let cpu::Cpu {
		a, x, y, s, p, pc, ..
	} = state.cpu;
	let inst = state.next_inst();

	// fceux flag order: N V - B D I Z C
	let n = if p.n() { 'N' } else { 'n' };
	let v = if p.v() { 'V' } else { 'v' };
	let u = 'u'; // always unused/reserved
	let b = if p.b() { 'B' } else { 'b' };
	let d = if p.d() { 'D' } else { 'd' };
	let i = if p.i() { 'I' } else { 'i' };
	let z = if p.z() { 'Z' } else { 'z' };
	let c = if p.c() { 'C' } else { 'c' };

	let byte_str = {
		let mut s = String::new();
		for offset in 0..inst.len() {
			let mem = state.mem_pure(state.cpu.pc + offset as u16);
			write!(&mut s, "{mem:02X} ").unwrap();
		}
		s.pop();
		s
	};

	let width = (0xFF - s) as usize;

	let frames = state.ppu.frame;
	let cycles = state.cycles;
	assert!(cycles.checked_mul(3).is_none() || cycles * 3 == state.ppu.cycles); // Implication operator when?

	let mut out = format!(
		"f{:<5} c{:<10} A:{:02X} X:{:02X} Y:{:02X} S:{:02X} {} {:width$}${:04X}: {:<9}",
		frames,
		cycles,
		a,
		x,
		y,
		s,
		format!("{n}{v}{u}{b}{d}{i}{z}{c}"),
		"",
		pc,
		byte_str,
	);

	print_instruction(state, &mut out).unwrap();

	out
}

macro_rules! make_log_test {
	($name:ident, $game:expr, $log:expr) => {
		#[test]
		fn $name() {
			use std::fs::File;
			use std::io::{BufRead, BufReader};

			let buffer = std::fs::read($game).unwrap();
			let game = Mapper::parse_ines(buffer).unwrap();
			let mut state = State::new(game, drawing::new_bitmap());
			let file = File::open($log).unwrap();
			let reader = BufReader::new(file);

			for (i, line) in reader.lines().enumerate() {
				let i = i + 1;
				let line = line.unwrap();
				let ours = fceux_log(&state);
				let debug_state = crate::display(&state);
				assert_eq!(
					ours, line,
					"Mismatch at line {i}\n ours: {ours}\n ref : {line}\n{debug_state}"
				);
				state.next();
			}
		}
	};
}

make_log_test!(
	fceux_log_2,
	"non-free/SMB1.nes",
	"reference-logs/SMB1-2.log"
);
