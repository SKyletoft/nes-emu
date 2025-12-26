use std::fmt::{self, Write};

use crate::{cpu, graphics, inst::Inst, interpret::State, mapper::Mapper, nrom128::NROM128, nrom256::NROM256};

fn print_instruction<M: Mapper>(state: &State<M>, f: &mut String) -> fmt::Result {
	let instruction = state.next_inst_pure();
	match instruction {
		Inst::AdcAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ADC ${:04X} = ${:02X}", adr, mem)
		}
		Inst::AdcAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "ADC ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::AdcAbsoluteY(adr) => {
			let res = adr.as_u16() + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "ADC ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::AdcImmediate(val) => write!(f, "ADC #${:02X}", val),
		Inst::AdcIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ADC (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::AdcIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ADC (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::AdcZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ADC ${:02X} = ${:02X}", adr, mem)
		}
		Inst::AdcZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "ADC ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::AhxAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AHX ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::AhxIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "AHX (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::AlrImmediate(val) => write!(f, "ALR ${:02X}", val),
		Inst::AncImmediate2(val) => write!(f, "ANC ${:02X}", val),
		Inst::AncImmediate(val) => write!(f, "ANC ${:02X}", val),
		Inst::AndAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "AND ${:04X} = ${:02X}", adr, mem)
		}
		Inst::AndAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "AND ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::AndAbsoluteY(adr) => {
			let res = adr.as_u16() + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "AND ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::AndImmediate(val) => write!(f, "AND #${:02X}", val),
		Inst::AndIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "AND (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::AndIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "AND (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::AndZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "AND ${:02X} = ${:02X}", adr, mem)
		}
		Inst::AndZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "AND ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::ArrImmediate(val) => write!(f, "ARR ${:02X}", val),
		Inst::AslAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ASL ${:04X} = ${:02X}", adr, mem)
		}
		Inst::AslAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ASL ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::AslAccumulator => write!(f, "ASL A"),
		Inst::AslZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ASL ${:02X} = ${:02X}", adr, mem)
		}
		Inst::AslZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ASL ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::AxsImmediate(val) => write!(f, "AXS ${:02X}", val),
		Inst::Bcc(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BCC ${:04X}", target)
		}
		Inst::Bcs(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BCS ${:04X}", target)
		}
		Inst::Beq(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BEQ ${:04X}", target)
		}
		Inst::BitAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "BIT ${:04X} = ${:02X}", adr, mem)
		}
		Inst::BitZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "BIT ${:02X} = ${:02X}", adr, mem)
		}
		Inst::Bmi(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BMI ${:04X}", target)
		}
		Inst::Bne(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BNE ${:04X}", target)
		}
		Inst::Bpl(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BPL ${:04X}", target)
		}
		Inst::Brk => write!(f, "BRK"),
		Inst::Bvc(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BVC ${:04X}", target)
		}
		Inst::Bvs(offset) => {
			let target = state
				.cpu
				.pc
				.wrapping_add(2)
				.wrapping_add(offset as i16 as u16);
			write!(f, "BVS ${:04X}", target)
		}
		Inst::Clc => write!(f, "CLC"),
		Inst::Cld => write!(f, "CLD"),
		Inst::Cli => write!(f, "CLI"),
		Inst::Clv => write!(f, "CLV"),
		Inst::CmpAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CMP ${:04X} = ${:02X}", adr, mem)
		}
		Inst::CmpAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "CMP ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::CmpAbsoluteY(adr) => {
			let res = adr.as_u16() + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "CMP ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::CmpImmediate(val) => write!(f, "CMP #${:02X}", val),
		Inst::CmpIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "CMP (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::CmpIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "CMP (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::CmpZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "CMP ${:02X} = ${:02X}", adr, mem)
		}
		Inst::CmpZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "CMP ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::CpxAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CPX ${:04X} = ${:02X}", adr, mem)
		}
		Inst::CpxImmediate(val) => write!(f, "CPX #${:02X}", val),
		Inst::CpxZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "CPX ${:02X} = ${:02X}", adr, mem)
		}
		Inst::CpyAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "CPY ${:04X} = ${:02X}", adr, mem)
		}
		Inst::CpyImmediate(val) => write!(f, "CPY #${:02X}", val),
		Inst::CpyZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "CPY ${:02X} = ${:02X}", adr, mem)
		}
		Inst::DcpAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X} = ${:02X}", adr, mem)
		}
		Inst::DcpAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::DcpAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DCP ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::DcpIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "DCP (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::DcpIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "DCP (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::DcpZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "DCP ${:02X} = ${:02X}", adr, mem)
		}
		Inst::DcpZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "DCP ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::DecAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "DEC ${:04X} = ${:02X}", adr, mem)
		}
		Inst::DecAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "DEC ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::DecZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "DEC ${:02X} = ${:02X}", adr, mem)
		}
		Inst::DecZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "DEC ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::Dex => write!(f, "DEX"),
		Inst::Dey => write!(f, "DEY"),
		Inst::EorAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X} = ${:02X}", adr, mem)
		}
		Inst::EorAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::EorAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "EOR ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::EorImmediate(val) => write!(f, "EOR #${:02X}", val),
		Inst::EorIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "EOR (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::EorIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "EOR (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::EorZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "EOR ${:02X} = ${:02X}", adr, mem)
		}
		Inst::EorZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "EOR ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::IncAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "INC ${:04X} = ${:02X}", adr, mem)
		}
		Inst::IncAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "INC ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::IncZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "INC ${:02X} = ${:02X}", adr, mem)
		}
		Inst::IncZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "INC ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::Inx => write!(f, "INX"),
		Inst::Iny => write!(f, "INY"),
		Inst::IscAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X} = ${:02X}", adr, mem)
		}
		Inst::IscAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::IscAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "ISC ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::IscIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ISC (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::IscIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ISC (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::IscZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ISC ${:02X} = ${:02X}", adr, mem)
		}
		Inst::IscZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ISC ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::JmpAbsolute(adr) => write!(f, "JMP ${:04X}", adr),
		Inst::JmpIndirect(adr) => {
			let adr = adr.as_u16();
			let lo = state.mem_pure(adr);
			let hi = if adr & 0xFF == 0xFF {
				state.mem_pure(adr & 0xFF00)
			} else {
				state.mem_pure(adr + 1)
			};
			let res = u16::from_be_bytes([hi, lo]);
			let mem = state.mem_pure(res);
			write!(f, "JMP (${adr:04X}) [${res:04X}] = ${mem:02X}")
		}
		Inst::Jsr(adr) => write!(f, "JSR ${:04X}", adr),
		Inst::LasAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAS ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::LaxAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAX ${:04X} = ${:02X}", adr, mem)
		}
		Inst::LaxAbsoluteY(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LAX ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::LaxImmediate(val) => write!(f, "LAX ${:02X}", val),
		Inst::LaxIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LAX (${:02X},X) = ${:02X}", adr, mem)
		}
		Inst::LaxIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LAX (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::LaxZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LAX ${:02X} = ${:02X}", adr, mem)
		}
		Inst::LaxZeroPageY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LAX ${:02X},Y = ${:02X}", adr, mem)
		}
		Inst::LdaAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDA ${:04X} = ${:02X}", adr, mem)
		}
		Inst::LdaAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "LDA ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::LdaAbsoluteY(adr) => {
			let res = adr.as_u16() + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "LDA ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::LdaImmediate(val) => write!(f, "LDA #${:02X}", val),
		Inst::LdaIndirectX(adr) => {
			let zp_adr = adr.wrapping_add(state.cpu.x);
			let lo = state.mem_pure(zp_adr as u16);
			let hi = state.mem_pure(zp_adr.wrapping_add(1) as u16);
			let eff_adr = u16::from_le_bytes([lo, hi]);
			let mem = state.mem_pure(eff_adr);
			write!(
				f,
				"LDA (${adr:02X},X) ${zp_adr:02X} ${eff_adr:04X} = ${mem:02X}"
			)
		}
		Inst::LdaIndirectY(adr) => {
			let lo = state.mem_pure(adr as u16);
			let hi = state.mem_pure(adr.wrapping_add(1) as u16);
			let res = u16::from_le_bytes([lo, hi]).wrapping_add(state.cpu.y as u16);
			let mem = state.mem_pure(res);
			write!(f, "LDA (${adr:02X}),Y [${res:04X}] = ${mem:02X}")
		}
		Inst::LdaZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LDA ${:02X} = ${:02X}", adr, mem)
		}
		Inst::LdaZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr) as u16;
			let mem = state.mem_pure(res);
			write!(f, "LDA ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::LdxAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDX ${:04X} = ${:02X}", adr, mem)
		}
		Inst::LdxAbsoluteY(adr) => {
			let eff_addr = adr.as_u16().wrapping_add(state.cpu.y as u16);
			let mem = state.mem_pure(eff_addr);
			write!(f, "LDX ${:04X},Y [${:04X}] = ${:02X}", adr, eff_addr, mem)
		}
		Inst::LdxImmediate(val) => write!(f, "LDX #${:02X}", val),
		Inst::LdxZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LDX ${:02X} = ${:02X}", adr, mem)
		}
		Inst::LdxZeroPageY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LDX ${:02X},Y = ${:02X}", adr, mem)
		}
		Inst::LdyAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LDY ${:04X} = ${:02X}", adr, mem)
		}
		Inst::LdyAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "LDY ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::LdyImmediate(val) => write!(f, "LDY #${:02X}", val),
		Inst::LdyZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LDY ${:02X} = ${:02X}", adr, mem)
		}
		Inst::LdyZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "LDY ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::LsrAccumulator => write!(f, "LSR A"),
		Inst::LsrAbsolute(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LSR ${:04X} = ${:02X}", adr, mem)
		}
		Inst::LsrAbsoluteX(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "LSR ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::LsrZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LSR ${:02X} = ${:02X}", adr, mem)
		}
		Inst::LsrZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "LSR ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::Nop2 => write!(f, "NOP"),
		Inst::Nop3 => write!(f, "NOP"),
		Inst::Nop4 => write!(f, "NOP"),
		Inst::Nop5 => write!(f, "NOP"),
		Inst::Nop6 => write!(f, "NOP"),
		Inst::Nop7 => write!(f, "NOP"),
		Inst::Ign(adr) => {
			let mem = state.mem_pure(adr.into());
			write!(f, "NOP ${:04X} = ${:02X}", adr, mem)
		}
		Inst::OraAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ORA ${:04X} = ${:02X}", adr, mem)
		}
		Inst::OraAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16.as_u16();
			let res = adr.wrapping_add(state.cpu.x as u16);
			let mem = state.mem_pure(res);
			write!(f, "ORA ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::OraAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16.as_u16();
			let res = adr.wrapping_add(state.cpu.y as u16);
			let mem = state.mem_pure(res);
			write!(f, "ORA ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::OraImmediate(val) => write!(f, "ORA #${:02X}", val),
		Inst::OraIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ORA (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::OraIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ORA (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::OraZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ORA ${:02X} = ${:02X}", adr, mem)
		}
		Inst::OraZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ORA ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::Pha => write!(f, "PHA"),
		Inst::Php => write!(f, "PHP"),
		Inst::Pla => write!(f, "PLA"),
		Inst::Plp => write!(f, "PLP"),
		Inst::RlaAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X} = ${:02X}", adr, mem)
		}
		Inst::RlaAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::RlaAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RLA ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::RlaIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RLA (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::RlaIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RLA (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::RlaZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RLA ${:02X} = ${:02X}", adr, mem)
		}
		Inst::RlaZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RLA ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::RolAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROL ${:04X} = ${:02X}", adr, mem)
		}
		Inst::RolAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROL ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::RolAccumulator => write!(f, "ROL A"),
		Inst::RolZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ROL ${:02X} = ${:02X}", adr, mem)
		}
		Inst::RolZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ROL ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::RorAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "ROR ${:04X} = ${:02X}", adr, mem)
		}
		Inst::RorAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(adr);
			write!(f, "ROR ${unaligned_u16:04X},X [${adr:04X}] = ${mem:02X}")
		}
		Inst::RorAccumulator => write!(f, "ROR A"),
		Inst::RorZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ROR ${:02X} = ${:02X}", adr, mem)
		}
		Inst::RorZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "ROR ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::RraAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X} = ${:02X}", adr, mem)
		}
		Inst::RraAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::RraAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "RRA ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::RraIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RRA (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::RraIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RRA (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::RraZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RRA ${:02X} = ${:02X}", adr, mem)
		}
		Inst::RraZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "RRA ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::Rti => write!(f, "RTI"),
		Inst::Rts => write!(f, "RTS"),
		Inst::SaxAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SAX ${:04X} = ${:02X}", adr, mem)
		}
		Inst::SaxIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SAX (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::SaxZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SAX ${:02X} = ${:02X}", adr, mem)
		}
		Inst::SaxZeroPageY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SAX ${:02X},Y = ${:02X}", adr, mem)
		}
		Inst::SbcAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SBC ${:04X} = ${:02X}", adr, mem)
		}
		Inst::SbcAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SBC ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::SbcAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16.as_u16();
			let res = adr + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "SBC ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::SbcImmediate(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcImmediate2(val) => write!(f, "SBC #${:02X}", val),
		Inst::SbcIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SBC (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::SbcIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SBC (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::SbcZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SBC ${:02X} = ${:02X}", adr, mem)
		}
		Inst::SbcZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "SBC ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::Sec => write!(f, "SEC"),
		Inst::Sed => write!(f, "SED"),
		Inst::Sei => write!(f, "SEI"),
		Inst::ShxAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SHX ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::ShyAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SHY ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::SloAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X} = ${:02X}", adr, mem)
		}
		Inst::SloAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::SloAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SLO ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::SloIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SLO (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::SloIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SLO (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::SloZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SLO ${:02X} = ${:02X}", adr, mem)
		}
		Inst::SloZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SLO ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::SreAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X} = ${:02X}", adr, mem)
		}
		Inst::SreAbsoluteX(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X},X = ${:02X}", adr, mem)
		}
		Inst::SreAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "SRE ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::SreIndirectX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SRE (${:02X}),X = ${:02X}", adr, mem)
		}
		Inst::SreIndirectY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SRE (${:02X}),Y = ${:02X}", adr, mem)
		}
		Inst::SreZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SRE ${:02X} = ${:02X}", adr, mem)
		}
		Inst::SreZeroPageX(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "SRE ${:02X},X = ${:02X}", adr, mem)
		}
		Inst::StaAbsolute(x) => {
			let mem = state.mem_pure(x.into());
			write!(f, "STA ${:04X} = ${:02X}", x, mem)
		}
		Inst::StaAbsoluteX(adr) => {
			let res = adr.as_u16() + state.cpu.x as u16;
			let mem = state.mem_pure(res);
			write!(f, "STA ${adr:04X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::StaAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16.as_u16();
			let res = adr + state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "STA ${adr:04X},Y [${res:04X}] = ${mem:02X}")
		}
		Inst::StaIndirectX(adr) => {
			let res = u16::from_le_bytes([
				state.mem_pure((adr + state.cpu.x) as u16),
				state.mem_pure((adr + state.cpu.x + 1) as u16),
			]);
			let mem = state.mem_pure(res);
			write!(f, "STA (${adr:02X}),X [${res:04X}] = ${mem:02X}")
		}
		Inst::StaIndirectY(adr) => {
			let res =
				u16::from_le_bytes([state.mem_pure(adr as u16), state.mem_pure(adr as u16 + 1)])
					+ state.cpu.y as u16;
			let mem = state.mem_pure(res);
			write!(f, "STA (${adr:02X}),Y [${res:04X}] = ${mem:02X}")
		}
		Inst::StaZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "STA ${:02X} = ${:02X}", adr, mem)
		}
		Inst::StaZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "STA ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::StxAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STX ${:04X} = ${:02X}", adr, mem)
		}
		Inst::StxZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "STX ${:02X} = ${:02X}", adr, mem)
		}
		Inst::StxZeroPageY(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "STX ${:02X},Y = ${:02X}", adr, mem)
		}
		Inst::StyAbsolute(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "STY ${:04X} = ${:02X}", adr, mem)
		}
		Inst::StyZeroPage(adr) => {
			let mem = state.mem_pure(adr as u16);
			write!(f, "STY ${:02X} = ${:02X}", adr, mem)
		}
		Inst::StyZeroPageX(adr) => {
			let res = state.cpu.x.wrapping_add(adr);
			let mem = state.mem_pure(res as u16);
			write!(f, "STY ${adr:02X},X [${res:04X}] = ${mem:02X}")
		}
		Inst::TasAbsoluteY(unaligned_u16) => {
			let adr = unaligned_u16;
			let mem = state.mem_pure(adr.into());
			write!(f, "TAS ${:04X},Y = ${:02X}", adr, mem)
		}
		Inst::Tax => write!(f, "TAX"),
		Inst::Tay => write!(f, "TAY"),
		Inst::Tsx => write!(f, "TSX"),
		Inst::Txa => write!(f, "TXA"),
		Inst::Txs => write!(f, "TXS"),
		Inst::Tya => write!(f, "TYA"),
		Inst::XaaImmediate(val) => write!(f, "XAA ${:02X}", val),

		_ => panic!("Unsupported unofficial instruction"),
	}
}

fn mesen_log<M: Mapper>(state: &State<M>, out: &mut String) {
	let cpu::Cpu { a, x, y, s, p, pc } = state.cpu;
	let stack_depth = (0xFF - s) as usize / 2;
	let inst = {
		let mut s = String::new();
		for _ in 0..stack_depth {
			s.push(' ');
		}
		print_instruction(state, &mut s).unwrap();
		s
	};
	let n = if p.n() { 'N' } else { 'n' };
	let v = if p.v() { 'V' } else { 'v' };
	let d = if p.d() { 'D' } else { 'd' };
	let i = if p.i() { 'I' } else { 'i' };
	let z = if p.z() { 'Z' } else { 'z' };
	let c = if p.c() { 'C' } else { 'c' };
	let scanline = state.ppu.scanline;
	let dot = state.ppu.dot;
	let frame = state.ppu.frame;
	let cycle = state.cycles;

	write!(
		out,
		"{pc:4X}  {inst:<32} A:{a:02X} X:{x:02X} Y:{y:02X} S:{s:02X} P:{n}{v}--{d}{i}{z}{c} V:{scanline:<3} H:{dot:<3} Fr:{frame} Cycle:{cycle}",
	).unwrap();
}

macro_rules! make_log_test {
	($name:ident, $game:expr, $log:expr, $mapper_type:ty) => {
		make_log_test!($name, $game, $log, $mapper_type, (|_: &mut State<$mapper_type> | {}));
	};
	($name:ident, $game:expr, $log:expr, $mapper_type:ty, $post_setup:expr) => {
		#[test]
		fn $name() {
			use std::{
				fs::File,
				io::{BufRead, BufReader},
			};

			let buffer = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), $game)).unwrap();
			let game = <$mapper_type>::parse_ines(&buffer).unwrap();
			let mut state = State::new(game, graphics::new_bitmap());
			let reader =
				BufReader::new(File::open(concat!(env!("CARGO_MANIFEST_DIR"), $log)).unwrap());
			let backup_reader =
				BufReader::new(File::open(concat!(env!("CARGO_MANIFEST_DIR"), $log)).unwrap());
			let mut ours = String::new();

			$post_setup(&mut state);

			for (i, line) in reader.lines().enumerate() {
				let i = i + 1;
				let line = line.unwrap();
				let _ = state.next_inst();

				ours.clear();
				mesen_log(&state, &mut ours);

				// Mesen's disassembly disagrees with its debugger when reading the APU status register
				assert!(
					ours == line
						|| (ours.contains("STA $4015 = ") && line.contains("STA $4015 = "))
						|| (ours.contains("STA $4016 = ") && line.contains("STA $4016 = "))
						|| (ours.contains("STA $4017 = ") && line.contains("STA $4017 = "))
						|| (ours.contains("STA $2007 = ") && line.contains("STA $2007 = "))
						|| (ours.contains("LDA $4016") && line.contains("LDA $4016"))
						|| (ours.contains("LDA $4017") && line.contains("LDA $4017")),
					"Mismatch at\n{}:{i}:\n ours: {ours}\n ref : {line}\n       {}\n{}\nPrev:\n{}",
					$log,
					ours.chars()
						.zip(line.chars())
						.map(|(l, r)| if l == r { ' ' } else { '^' })
						.chain(std::iter::repeat('^'))
						.take(ours.len().max(line.len()))
						.collect::<String>(),
					state.display(),
					backup_reader.lines().nth(i - 2).unwrap().unwrap(),
				);
				if state.cycles == 116745 {
					assert_eq!(state.mem_pure(0x01FF), 0x80, "\n{}", state.display());
					assert_eq!(state.mem_pure(0x01FE), 0x57, "\n{}", state.display());
					assert_eq!(state.mem_pure(0x01FD), 0xA5, "\n{}", state.display());
					println!("Stack check passed");
				}
				state.next();
			}
		}
	};
}

make_log_test!(
	mesen_log_1,
	"/../non-free/SMB1.nes",
	"/../reference-logs/SMB1-Mesen.txt",
	NROM256
);

make_log_test!(
	mesen_log_2,
	"/../non-free/SMB1.nes",
	"/../reference-logs/SMB1-Mesen-long.log",
	NROM256
);
