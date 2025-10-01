mod cpu;
mod drawing;
mod evaluate_instruction;
mod inst;
mod interpret;
mod nes_file;
mod ppu;

use std::{fmt, sync::{Arc, Mutex}};
#[cfg(test)]
use std::fmt::Write;

use drawing::Bitmap;
use inst::Inst;
use interpret::State;
use nes_file::Mapper;

fn display(state: &State) {
	let cpu::Cpu {
		a, x, y, s, p, pc, ..
	} = state.cpu;

	let c = p.c() as u8;
	let z = p.z() as u8;
	let i = p.i() as u8;
	let d = p.d() as u8;
	let b = p.b() as u8;
	let v = p.v() as u8;
	let n = p.n() as u8;

	let inst = state.next_inst();

	let line = state.ppu.scanline;
	let dot = state.ppu.dot;
	let frame = state.ppu.frame % 10000;

	println!("┌─CPU──────────────────────────┐");
	println!("│ A:{a:02X} X:{x:02X} Y:{y:02X} SP:{s:02X} pc:{pc:04X} │");
	println!("│ C:{c} Z:{z} I:{i} D:{d} B:{b} V:{v} N:{n}  │");
	println!("├─PPU──────────────────────────┤");
	println!("│ line:{line:03} dot:{dot:03} frame: {frame:04} │");
	println!("└──────────────────────────────┘");
	println!("Next: {inst:X?}");
	println!();
}

#[cfg(test)]
fn print_instruction(instruction: Inst, f: &mut String) -> fmt::Result {
	match instruction {
		Inst::AdcAbsolute(addr) => write!(f, "ADC ${:04X}", addr),
		Inst::AdcAbsoluteX(addr) => write!(f, "ADC ${:04X},X", addr),
		Inst::AdcAbsoluteY(addr) => write!(f, "ADC ${:04X},Y", addr),
		Inst::AdcImmediate(val) => write!(f, "ADC #${:02X}", val),
		Inst::AdcIndirectX(addr) => write!(f, "ADC (${:02X}),X", addr),
		Inst::AdcIndirectY(addr) => write!(f, "ADC (${:02X}),Y", addr),
		Inst::AdcZeroPage(addr) => write!(f, "ADC ${:02X}", addr),
		Inst::AdcZeroPageX(addr) => write!(f, "ADC ${:02X},X", addr),
		Inst::AhxAbsoluteY(addr) => write!(f, "AHX ${:04X},Y", addr),
		Inst::AhxIndirectY(addr) => write!(f, "AHX (${:02X}),Y", addr),
		Inst::AlrImmediate(val) => write!(f, "ALR #${:02X}", val),
		Inst::AncImmediate2(val) => write!(f, "ANC #${:02X}", val),
		Inst::AncImmediate(val) => write!(f, "ANC #${:02X}", val),
		Inst::AndAbsolute(addr) => write!(f, "AND ${:04X}", addr),
		Inst::AndAbsoluteX(addr) => write!(f, "AND ${:04X},X", addr),
		Inst::AndAbsoluteY(addr) => write!(f, "AND ${:04X},Y", addr),
		Inst::AndImmediate(val) => write!(f, "AND #${:02X}", val),
		Inst::AndIndirectX(addr) => write!(f, "AND (${:02X}),X", addr),
		Inst::AndIndirectY(addr) => write!(f, "AND (${:02X}),Y", addr),
		Inst::AndZeroPage(addr) => write!(f, "AND ${:02X}", addr),
		Inst::AndZeroPageX(addr) => write!(f, "AND ${:02X},X", addr),
		Inst::ArrImmediate(val) => write!(f, "ARR #${:02X}", val),
		Inst::AslAbsolute(addr) => write!(f, "ASL ${:04X}", addr),
		Inst::AslAbsoluteX(addr) => write!(f, "ASL ${:04X},X", addr),
		Inst::AslAccumulator => write!(f, "ASL A"),
		Inst::AslZeroPage(addr) => write!(f, "ASL ${:02X}", addr),
		Inst::AslZeroPageX(addr) => write!(f, "ASL ${:02X},X", addr),
		Inst::AxsImmediate(val) => write!(f, "AXS #${:02X}", val),
		Inst::Bcc(offset) => write!(f, "BCC ${:02X}", offset),
		Inst::Bcs(offset) => write!(f, "BCS ${:02X}", offset),
		Inst::Beq(offset) => write!(f, "BEQ ${:02X}", offset),
		Inst::BitAbsolute(addr) => write!(f, "BIT ${:04X}", addr),
		Inst::BitZeroPage(addr) => write!(f, "BIT ${:02X}", addr),
		Inst::Bmi(offset) => write!(f, "BMI ${:02X}", offset),
		Inst::Bne(offset) => write!(f, "BNE ${:02X}", offset),
		Inst::Bpl(offset) => write!(f, "BPL ${:02X}", offset),
		Inst::Brk => write!(f, "BRK"),
		Inst::Bvc(offset) => write!(f, "BVC ${:02X}", offset),
		Inst::Bvs(offset) => write!(f, "BVS ${:02X}", offset),
		Inst::Clc => write!(f, "CLC"),
		Inst::Cld => write!(f, "CLD"),
		Inst::Cli => write!(f, "CLI"),
		Inst::Clv => write!(f, "CLV"),
		Inst::CmpAbsolute(addr) => write!(f, "CMP ${:04X}", addr),
		Inst::CmpAbsoluteX(addr) => write!(f, "CMP ${:04X},X", addr),
		Inst::CmpAbsoluteY(addr) => write!(f, "CMP ${:04X},Y", addr),
		Inst::CmpImmediate(val) => write!(f, "CMP #${:02X}", val),
		Inst::CmpIndirectX(addr) => write!(f, "CMP (${:02X}),X", addr),
		Inst::CmpIndirectY(addr) => write!(f, "CMP (${:02X}),Y", addr),
		Inst::CmpZeroPage(addr) => write!(f, "CMP ${:02X}", addr),
		Inst::CmpZeroPageX(addr) => write!(f, "CMP ${:02X},X", addr),
		Inst::CpxAbsolute(addr) => write!(f, "CPX ${:04X}", addr),
		Inst::CpxImmediate(val) => write!(f, "CPX #${:02X}", val),
		Inst::CpxZeroPage(addr) => write!(f, "CPX ${:02X}", addr),
		Inst::CpyAbsolute(addr) => write!(f, "CPY ${:04X}", addr),
		Inst::CpyImmediate(val) => write!(f, "CPY #${:02X}", val),
		Inst::CpyZeroPage(addr) => write!(f, "CPY ${:02X}", addr),
		Inst::DCPAbsolute(addr) => write!(f, "DCP ${:04X}", addr),
		Inst::DCPAbsoluteX(addr) => write!(f, "DCP ${:04X},X", addr),
		Inst::DCPAbsoluteY(addr) => write!(f, "DCP ${:04X},Y", addr),
		Inst::DCPIndirectX(addr) => write!(f, "DCP (${:02X}),X", addr),
		Inst::DCPIndirectY(addr) => write!(f, "DCP (${:02X}),Y", addr),
		Inst::DCPZeroPage(addr) => write!(f, "DCP ${:02X}", addr),
		Inst::DCPZeroPageX(addr) => write!(f, "DCP ${:02X},X", addr),
		Inst::DecAbsolute(addr) => write!(f, "DEC ${:04X}", addr),
		Inst::DecAbsoluteX(addr) => write!(f, "DEC ${:04X},X", addr),
		Inst::DecZeroPage(addr) => write!(f, "DEC ${:02X}", addr),
		Inst::DecZeroPageX(addr) => write!(f, "DEC ${:02X},X", addr),
		Inst::Dex => write!(f, "DEX"),
		Inst::Dey => write!(f, "DEY"),
		Inst::EorAbsolute(addr) => write!(f, "EOR ${:04X}", addr),
		Inst::EorAbsoluteX(addr) => write!(f, "EOR ${:04X},X", addr),
		Inst::EorAbsoluteY(addr) => write!(f, "EOR ${:04X},Y", addr),
		Inst::EorImmediate(val) => write!(f, "EOR #${:02X}", val),
		Inst::EorIndirectX(addr) => write!(f, "EOR (${:02X}),X", addr),
		Inst::EorIndirectY(addr) => write!(f, "EOR (${:02X}),Y", addr),
		Inst::EorZeroPage(addr) => write!(f, "EOR ${:02X}", addr),
		Inst::EorZeroPageX(addr) => write!(f, "EOR ${:02X},X", addr),
		Inst::IncAbsolute(addr) => write!(f, "INC ${:04X}", addr),
		Inst::IncAbsoluteX(addr) => write!(f, "INC ${:04X},X", addr),
		Inst::IncZeroPage(addr) => write!(f, "INC ${:02X}", addr),
		Inst::IncZeroPageX(addr) => write!(f, "INC ${:02X},X", addr),
		Inst::Inx => write!(f, "INX"),
		Inst::Iny => write!(f, "INY"),
		Inst::ISCAbsolute(addr) => write!(f, "ISC ${:04X}", addr),
		Inst::ISCAbsoluteX(addr) => write!(f, "ISC ${:04X},X", addr),
		Inst::ISCAbsoluteY(addr) => write!(f, "ISC ${:04X},Y", addr),
		Inst::ISCIndirectX(addr) => write!(f, "ISC (${:02X}),X", addr),
		Inst::ISCIndirectY(addr) => write!(f, "ISC (${:02X}),Y", addr),
		Inst::ISCZeroPage(addr) => write!(f, "ISC ${:02X}", addr),
		Inst::ISCZeroPageX(addr) => write!(f, "ISC ${:02X},X", addr),
		Inst::JmpAbsolute(addr) => write!(f, "JMP ${:04X}", addr),
		Inst::JmpIndirect(addr) => write!(f, "JMP (${:04X})", addr),
		Inst::Jsr(addr) => write!(f, "JSR ${:04X}", addr),
		Inst::LASAbsoluteY(addr) => write!(f, "LAS ${:04X},Y", addr),
		Inst::LAXAbsolute(addr) => write!(f, "LAX ${:04X}", addr),
		Inst::LAXAbsoluteY(addr) => write!(f, "LAX ${:04X},Y", addr),
		Inst::LAXImmediate(val) => write!(f, "LAX #${:02X}", val),
		Inst::LAXIndirectX(addr) => write!(f, "LAX (${:02X},X)", addr),
		Inst::LAXIndirectY(addr) => write!(f, "LAX (${:02X}),Y", addr),
		Inst::LAXZeroPage(addr) => write!(f, "LAX ${:02X}", addr),
		Inst::LAXZeroPageY(addr) => write!(f, "LAX ${:02X},Y", addr),
		Inst::LdaAbsolute(addr) => write!(f, "LDA ${:04X}", addr),
		Inst::LdaAbsoluteX(addr) => write!(f, "LDA ${:04X},X", addr),
		Inst::LdaAbsoluteY(addr) => write!(f, "LDA ${:04X},Y", addr),
		Inst::LdaImmediate(val) => write!(f, "LDA #${:02X}", val),
		Inst::LdaIndirectX(addr) => write!(f, "LDA (${:02X},X)", addr),
		Inst::LdaIndirectY(addr) => write!(f, "LDA (${:02X}),Y", addr),
		Inst::LdaZeroPage(addr) => write!(f, "LDA ${:02X}", addr),
		Inst::LdaZeroPageX(addr) => write!(f, "LDA ${:02X},X", addr),
		Inst::LdxAbsolute(addr) => write!(f, "LDX ${:04X}", addr),
		Inst::LdxAbsoluteY(addr) => write!(f, "LDX ${:04X},Y", addr),
		Inst::LdxImmediate(val) => write!(f, "LDX #${:02X}", val),
		Inst::LdxZeroPage(addr) => write!(f, "LDX ${:02X}", addr),
		Inst::LdxZeroPageY(addr) => write!(f, "LDX ${:02X},Y", addr),
		Inst::LdyAbsolute(addr) => write!(f, "LDY ${:04X}", addr),
		Inst::LdyAbsoluteX(addr) => write!(f, "LDY ${:04X},X", addr),
		Inst::LdyImmediate(val) => write!(f, "LDY #${:02X}", val),
		Inst::LdyZeroPage(addr) => write!(f, "LDY ${:02X}", addr),
		Inst::LdyZeroPageX(addr) => write!(f, "LDY ${:02X},X", addr),
		Inst::LsrAccumulator => write!(f, "LSR A"),
		Inst::LsrAbsolute(addr) => write!(f, "LSR ${:04X}", addr),
		Inst::LsrAbsoluteX(addr) => write!(f, "LSR ${:04X},X", addr),
		Inst::LsrZeroPage(addr) => write!(f, "LSR ${:02X}", addr),
		Inst::LsrZeroPageX(addr) => write!(f, "LSR ${:02X},X", addr),
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
		Inst::NOPAbsolute(addr) => write!(f, "NOP ${:04X}", addr),
		Inst::NOPAbsoluteX(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPAbsoluteX2(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPAbsoluteX3(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPAbsoluteX4(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPAbsoluteX5(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPAbsoluteX6(addr) => write!(f, "NOP ${:04X},X", addr),
		Inst::NOPImmediate(val) => write!(f, "NOP #${:02X}", val),
		Inst::NOPImmediate2(val) => write!(f, "NOP #${:02X}", val),
		Inst::NOPImmediate3(val) => write!(f, "NOP"),
		Inst::NOPZeroPage(addr) => write!(f, "NOP ${:02X}", addr),
		Inst::NOPZeroPage3(addr) => write!(f, "NOP ${:02X}", addr),
		Inst::NOPZeroPage4(addr) => write!(f, "NOP ${:02X}", addr),
		Inst::NOPZeroPageX(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::NOPZeroPageX2(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::NOPZeroPageX3(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::NOPZeroPageX4(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::NOPZeroPageX5(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::NOPZeroPageX6(addr) => write!(f, "NOP ${:02X},X", addr),
		Inst::OraAbsolute(addr) => write!(f, "ORA ${:04X}", addr),
		Inst::OraAbsoluteX(addr) => write!(f, "ORA ${:04X},X", addr),
		Inst::OraAbsoluteY(addr) => write!(f, "ORA ${:04X},Y", addr),
		Inst::OraImmediate(val) => write!(f, "ORA #${:02X}", val),
		Inst::OraIndirectX(addr) => write!(f, "ORA (${:02X}),X", addr),
		Inst::OraIndirectY(addr) => write!(f, "ORA (${:02X}),Y", addr),
		Inst::OraZeroPage(addr) => write!(f, "ORA ${:02X}", addr),
		Inst::OraZeroPageX(addr) => write!(f, "ORA ${:02X},X", addr),
		Inst::Pha => write!(f, "PHA"),
		Inst::Php => write!(f, "PHP"),
		Inst::Pla => write!(f, "PLA"),
		Inst::Plp => write!(f, "PLP"),
		Inst::RLAAbsolute(addr) => write!(f, "RLA ${:04X}", addr),
		Inst::RLAAbsoluteX(addr) => write!(f, "RLA ${:04X},X", addr),
		Inst::RLAAbsoluteY(addr) => write!(f, "RLA ${:04X},Y", addr),
		Inst::RLAIndirectX(addr) => write!(f, "RLA (${:02X}),X", addr),
		Inst::RLAIndirectY(addr) => write!(f, "RLA (${:02X}),Y", addr),
		Inst::RLAZeroPage(addr) => write!(f, "RLA ${:02X}", addr),
		Inst::RLAZeroPageX(addr) => write!(f, "RLA ${:02X},X", addr),
		Inst::RolAbsolute(addr) => write!(f, "ROL ${:04X}", addr),
		Inst::RolAbsoluteX(addr) => write!(f, "ROL ${:04X},X", addr),
		Inst::RolAccumulator => write!(f, "ROL A"),
		Inst::RolZeroPage(addr) => write!(f, "ROL ${:02X}", addr),
		Inst::RolZeroPageX(addr) => write!(f, "ROL ${:02X},X", addr),
		Inst::RorAbsolute(addr) => write!(f, "ROR ${:04X}", addr),
		Inst::RorAbsoluteX(addr) => write!(f, "ROR ${:04X},X", addr),
		Inst::RorAccumulator => write!(f, "ROR A"),
		Inst::RorZeroPage(addr) => write!(f, "ROR ${:02X}", addr),
		Inst::RorZeroPageX(addr) => write!(f, "ROR ${:02X},X", addr),
		Inst::RRAAbsolute(addr) => write!(f, "RRA ${:04X}", addr),
		Inst::RRAAbsoluteX(addr) => write!(f, "RRA ${:04X},X", addr),
		Inst::RRAAbsoluteY(addr) => write!(f, "RRA ${:04X},Y", addr),
		Inst::RRAIndirectX(addr) => write!(f, "RRA (${:02X}),X", addr),
		Inst::RRAIndirectY(addr) => write!(f, "RRA (${:02X}),Y", addr),
		Inst::RRAZeroPage(addr) => write!(f, "RRA ${:02X}", addr),
		Inst::RRAZeroPageX(addr) => write!(f, "RRA ${:02X},X", addr),
		Inst::Rti => write!(f, "RTI"),
		Inst::Rts => write!(f, "RTS"),
		Inst::SAXAbsolute(addr) => write!(f, "SAX ${:04X}", addr),
		Inst::SAXIndirectX(addr) => write!(f, "SAX (${:02X}),X", addr),
		Inst::SAXZeroPage(addr) => write!(f, "SAX ${:02X}", addr),
		Inst::SAXZeroPageY(addr) => write!(f, "SAX ${:02X},Y", addr),
		Inst::SbcAbsolute(addr) => write!(f, "SBC ${:04X}", addr),
		Inst::SbcAbsoluteX(addr) => write!(f, "SBC ${:04X},X", addr),
		Inst::SbcAbsoluteY(addr) => write!(f, "SBC ${:04X},Y", addr),
		Inst::SbcImmediate(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcImmediate2(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcIndirectX(addr) => write!(f, "SBC (${:02X}),X", addr),
		Inst::SbcIndirectY(addr) => write!(f, "SBC (${:02X}),Y", addr),
		Inst::SbcZeroPage(addr) => write!(f, "SBC ${:02X}", addr),
		Inst::SbcZeroPageX(addr) => write!(f, "SBC ${:02X},X", addr),
		Inst::Sec => write!(f, "SEC"),
		Inst::Sed => write!(f, "SED"),
		Inst::Sei => write!(f, "SEI"),
		Inst::SHXAbsoluteY(addr) => write!(f, "SHX ${:04X},Y", addr),
		Inst::SHYAbsoluteX(addr) => write!(f, "SHY ${:04X},X", addr),
		Inst::SLOAbsolute(addr) => write!(f, "SLO ${:04X}", addr),
		Inst::SLOAbsoluteX(addr) => write!(f, "SLO ${:04X},X", addr),
		Inst::SLOAbsoluteY(addr) => write!(f, "SLO ${:04X},Y", addr),
		Inst::SLOIndirectX(addr) => write!(f, "SLO (${:02X}),X", addr),
		Inst::SLOIndirectY(addr) => write!(f, "SLO (${:02X}),Y", addr),
		Inst::SLOZeroPage(addr) => write!(f, "SLO ${:02X}", addr),
		Inst::SLOZeroPageX(addr) => write!(f, "SLO ${:02X},X", addr),
		Inst::SREAbsolute(addr) => write!(f, "SRE ${:04X}", addr),
		Inst::SREAbsoluteX(addr) => write!(f, "SRE ${:04X},X", addr),
		Inst::SREAbsoluteY(addr) => write!(f, "SRE ${:04X},Y", addr),
		Inst::SREIndirectX(addr) => write!(f, "SRE (${:02X}),X", addr),
		Inst::SREIndirectY(addr) => write!(f, "SRE (${:02X}),Y", addr),
		Inst::SREZeroPage(addr) => write!(f, "SRE ${:02X}", addr),
		Inst::SREZeroPageX(addr) => write!(f, "SRE ${:02X},X", addr),
		Inst::StaAbsolute(addr) => write!(f, "STA ${:04X}", addr),
		Inst::StaAbsoluteX(addr) => write!(f, "STA ${:04X},X", addr),
		Inst::StaAbsoluteY(addr) => write!(f, "STA ${:04X},Y", addr),
		Inst::StaIndirectX(addr) => write!(f, "STA (${:02X}),X", addr),
		Inst::StaIndirectY(addr) => write!(f, "STA (${:02X}),Y", addr),
		Inst::StaZeroPage(addr) => write!(f, "STA ${:02X}", addr),
		Inst::StaZeroPageX(addr) => write!(f, "STA ${:02X},X", addr),
		Inst::StxAbsolute(addr) => write!(f, "STX ${:04X}", addr),
		Inst::StxZeroPage(addr) => write!(f, "STX ${:02X}", addr),
		Inst::StxZeroPageY(addr) => write!(f, "STX ${:02X},Y", addr),
		Inst::StyAbsolute(addr) => write!(f, "STY ${:04X}", addr),
		Inst::StyZeroPage(addr) => write!(f, "STY ${:02X}", addr),
		Inst::StyZeroPageX(addr) => write!(f, "STY ${:02X},X", addr),
		Inst::TASAbsoluteY(addr) => write!(f, "TAS ${:04X},Y", addr),
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
		s.pop(); // Remove trailing space
		s
	};

	let mut out = format!(
		"A:{:02X} X:{:02X} Y:{:02X} S:{:02X} {}   ${:04X}:  {:<8}",
		a,
		x,
		y,
		s,
		format!("{n}{v}{u}{b}{d}{i}{z}{c}"),
		pc,
		byte_str,
	);

	// Add instruction
	let mut instruction_str = String::new();
	print_instruction(inst, &mut instruction_str).unwrap();
	out.push_str(&instruction_str);

	out
}

fn emulation_loop(shared_texture: Arc<Mutex<Bitmap>>) {
	let path = std::env::args()
		.nth(1)
		.unwrap_or_else(|| "../non-free/SMB1.nes".into());
	dbg!(&path);
	let buffer = std::fs::read(path).unwrap();
	let game = Mapper::parse_ines(buffer).unwrap();
	let mut system_state = State::new(game, shared_texture);

	// let mut buf = String::new();
	loop {
		system_state.next();

		display(&system_state);
		// buf.clear();
		// std::io::stdin().read_line(&mut buf).unwrap();
	}
}

fn main() {
	let shared_texture = drawing::new_bitmap();

	let texture_ptr = shared_texture.clone();
	let _emulation = std::thread::spawn(|| emulation_loop(texture_ptr));
	drawing::sdl_thread(shared_texture).unwrap();

	_emulation.join().unwrap();
}

#[cfg(test)]
mod test {
	use crate::{drawing, inst::Inst, interpret::State, nes_file::Mapper};

	#[test]
	fn smb3_first_few() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		assert_eq!(state.next_inst(), Inst::Sei);
		assert_eq!(state.cpu.pc, 0xFF40);
		state.next();
		assert_eq!(state.next_inst(), Inst::Cld);
		state.next();
		assert_eq!(state.next_inst(), Inst::LdaImmediate(0));
		assert_eq!(state.cpu.a, 0);
		assert!(!state.cpu.p.d()); // A bit late for some reason
		state.next();
		assert_eq!(state.next_inst(), Inst::StaAbsolute(0x2001u16.into()));
		state.next();
		assert_eq!(state.next_inst(), Inst::LdaImmediate(8));
		state.next();
		assert_eq!(state.next_inst(), Inst::StaAbsolute(0x2000u16.into()));
		assert_eq!(state.cpu.a, 8);
		assert_eq!(state.cpu.pc, 0xFF49);
		assert_eq!(state.cpu.s, 0xFD);
	}

	#[test]
	fn smb3_first_jsr() {
		let buffer = std::fs::read("non-free/SMB3.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		for _ in 0..7 {
			state.next();
		}
		// Wait for PPU to init...
		for i in 0..(25559 / 2) {
			assert_eq!(state.cpu.pc, 0xFF4E);
			assert_eq!(state.next_inst(), Inst::LdaAbsolute(0x2002u16.into()));
			state.next();
			assert_eq!(state.cpu.pc, 0xFF51);
			assert_eq!(state.next_inst(), Inst::Bpl(-5), "Loop: {i}");
			state.next();
		}
		assert_eq!(state.cpu.pc, 0xFF53);
		assert_eq!(state.next_inst(), Inst::Dex);
	}

	#[test]
	fn fceux_log_1() {
		use std::fs::File;
		use std::io::{BufRead, BufReader};

		let buffer = std::fs::read("non-free/SMB1.nes").unwrap();
		let game = Mapper::parse_ines(buffer).unwrap();
		let mut state = State::new(game, drawing::new_bitmap());
		let file = File::open("reference-logs/SMB1.log").unwrap();
		let reader = BufReader::new(file);

		for (i, line) in reader.lines().enumerate() {
			let line = line.unwrap();
			let ours = crate::fceux_log(&state);
			assert_eq!(
				ours, line,
				"Mismatch at line {i}\n ours: {ours}\n ref : {line}"
			);
			state.next();
		}
	}
}
