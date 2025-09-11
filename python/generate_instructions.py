# generate_instructions.py
# Generates Rust enums for all NES instructions (official + unofficial).

# opcode matrix source: NESDev Wiki "CPU unofficial opcodes"
# https://www.nesdev.org/wiki/CPU_unofficial_opcodes

instr_modes = {
	"ADC": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"AND": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"ASL": ["Accumulator","ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"BCC": ["Relative"],
	"BCS": ["Relative"],
	"BEQ": ["Relative"],
	"BIT": ["ZeroPage","Absolute"],
	"BMI": ["Relative"],
	"BNE": ["Relative"],
	"BPL": ["Relative"],
	"BRK": ["Implied"],
	"BVC": ["Relative"],
	"BVS": ["Relative"],
	"CLC": ["Implied"],
	"CLD": ["Implied"],
	"CLI": ["Implied"],
	"CLV": ["Implied"],
	"CMP": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"CPX": ["Immediate","ZeroPage","Absolute"],
	"CPY": ["Immediate","ZeroPage","Absolute"],
	"DEC": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"DEX": ["Implied"],
	"DEY": ["Implied"],
	"EOR": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"INC": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"INX": ["Implied"],
	"INY": ["Implied"],
	"JMP": ["Absolute","Indirect"],
	"JSR": ["Absolute"],
	"LDA": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"LDX": ["Immediate","ZeroPage","ZeroPageY","Absolute","AbsoluteY"],
	"LDY": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"LSR": ["Accumulator","ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"NOP": ["Implied"],
	"ORA": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"PHA": ["Implied"],
	"PHP": ["Implied"],
	"PLA": ["Implied"],
	"PLP": ["Implied"],
	"ROL": ["Accumulator","ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"ROR": ["Accumulator","ZeroPage","ZeroPageX","Absolute","AbsoluteX"],
	"RTI": ["Implied"],
	"RTS": ["Implied"],
	"SBC": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"SEC": ["Implied"],
	"SED": ["Implied"],
	"SEI": ["Implied"],
	"STA": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"STX": ["ZeroPage","ZeroPageY","Absolute"],
	"STY": ["ZeroPage","ZeroPageX","Absolute"],
	"TAX": ["Implied"],
	"TAY": ["Implied"],
	"TSX": ["Implied"],
	"TXA": ["Implied"],
	"TXS": ["Implied"],
	"TYA": ["Implied"],

	# Unofficials (examples; full table needs filling from nesdev wiki)
	"LAX": ["ZeroPage","ZeroPageY","Absolute","AbsoluteY","IndirectX","IndirectY"],
	"SAX": ["ZeroPage","ZeroPageY","Absolute","IndirectX"],
	"DCP": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"ISC": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"RLA": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"RRA": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"SLO": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"SRE": ["ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY","IndirectX","IndirectY"],
	"ANC": ["Immediate"],
	"ALR": ["Immediate"],
	"ARR": ["Immediate"],
	"AXS": ["Immediate"],
	"LAS": ["AbsoluteY"],
	"TAS": ["AbsoluteY"],
	"SHY": ["AbsoluteX"],
	"SHX": ["AbsoluteY"],
	"AHX": ["AbsoluteY","IndirectY"],
	"NOPU": ["Immediate","ZeroPage","ZeroPageX","Absolute","AbsoluteX","AbsoluteY"], # unofficial NOP variants
}

# Instructions that are conditional jumps (they modify PC)
conditional_jumps = {
	"BCC", "BCS", "BEQ", "BMI", "BNE", "BPL", "BVC", "BVS"
}

# Instruction lengths in bytes
instruction_lengths = {
	"ADC": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"AND": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"ASL": {"Accumulator": 1, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"BCC": {"Relative": 2},
	"BCS": {"Relative": 2},
	"BEQ": {"Relative": 2},
	"BIT": {"ZeroPage": 2, "Absolute": 3},
	"BMI": {"Relative": 2},
	"BNE": {"Relative": 2},
	"BPL": {"Relative": 2},
	"BRK": {"Implied": 1},
	"BVC": {"Relative": 2},
	"BVS": {"Relative": 2},
	"CLC": {"Implied": 1},
	"CLD": {"Implied": 1},
	"CLI": {"Implied": 1},
	"CLV": {"Implied": 1},
	"CMP": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"CPX": {"Immediate": 2, "ZeroPage": 2, "Absolute": 3},
	"CPY": {"Immediate": 2, "ZeroPage": 2, "Absolute": 3},
	"DEC": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"DEX": {"Implied": 1},
	"DEY": {"Implied": 1},
	"EOR": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"INC": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"INX": {"Implied": 1},
	"INY": {"Implied": 1},
	"JMP": {"Absolute": 3, "Indirect": 3},
	"JSR": {"Absolute": 3},
	"LDA": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"LDX": {"Immediate": 2, "ZeroPage": 2, "ZeroPageY": 2, "Absolute": 3, "AbsoluteY": 3},
	"LDY": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"LSR": {"Accumulator": 1, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"NOP": {"Implied": 1},
	"ORA": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"PHA": {"Implied": 1},
	"PHP": {"Implied": 1},
	"PLA": {"Implied": 1},
	"PLP": {"Implied": 1},
	"ROL": {"Accumulator": 1, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"ROR": {"Accumulator": 1, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3},
	"RTI": {"Implied": 1},
	"RTS": {"Implied": 1},
	"SBC": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"SEC": {"Implied": 1},
	"SED": {"Implied": 1},
	"SEI": {"Implied": 1},
	"STA": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"STX": {"ZeroPage": 2, "ZeroPageY": 2, "Absolute": 3},
	"STY": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3},
	"TAX": {"Implied": 1},
	"TAY": {"Implied": 1},
	"TSX": {"Implied": 1},
	"TXA": {"Implied": 1},
	"TXS": {"Implied": 1},
	"TYA": {"Implied": 1},

	# Unofficials
	"LAX": {"ZeroPage": 2, "ZeroPageY": 2, "Absolute": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"SAX": {"ZeroPage": 2, "ZeroPageY": 2, "Absolute": 3, "IndirectX": 2},
	"DCP": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"ISC": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"RLA": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"RRA": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"SLO": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"SRE": {"ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3, "IndirectX": 2, "IndirectY": 2},
	"ANC": {"Immediate": 2},
	"ALR": {"Immediate": 2},
	"ARR": {"Immediate": 2},
	"AXS": {"Immediate": 2},
	"LAS": {"AbsoluteY": 3},
	"TAS": {"AbsoluteY": 3},
	"SHY": {"AbsoluteX": 3},
	"SHX": {"AbsoluteY": 3},
	"AHX": {"AbsoluteY": 3, "IndirectY": 2},
	"NOPU": {"Immediate": 2, "ZeroPage": 2, "ZeroPageX": 2, "Absolute": 3, "AbsoluteX": 3, "AbsoluteY": 3},
}

print("#![allow(unused)]")
print("use crate::cpu::Cpu;")
print("use crate::evaluate_instruction::*;")
print()
print("// Auto-generated NES CPU instruction set")
print("#[derive(Debug, Copy, Clone, Eq, PartialEq)]\npub enum Inst {")
for instr, modes in instr_modes.items():
	if len(modes) == 1:
		m = modes[0]
		if m == "Immediate":
			print(f"\t{instr}(u8),")
		elif m == "Relative":
			print(f"\t{instr}(i8),")
		elif m == "Accumulator":
			print(f"\t{instr},")
		elif m == "Implied":
			print(f"\t{instr},")
		elif m in ("Absolute","AbsoluteX","AbsoluteY","Indirect"):
			print(f"\t{instr}(u8, u8),")
		else:
			print(f"\t{m}(u8),")
	else:
		print(f"\t{instr}({instr}),")
print("}\n")

for instr, modes in instr_modes.items():
	if len(modes) == 1:
		pass
	else:
		print(f"#[derive(Debug, Copy, Clone, Eq, PartialEq)]\npub enum {instr} {{")
		for m in modes:
			if m == "Immediate":
				print(f"\tImmediate(u8),")
			elif m == "Relative":
				print(f"\tRelative(i8),")
			elif m == "Accumulator":
				print(f"\tAccumulator,")
			elif m == "Implied":
				print(f"\tImplied,")
			elif m in ("Absolute","AbsoluteX","AbsoluteY","Indirect"):
				print(f"\t{m}(u8, u8),")
			else:
				print(f"\t{m}(u8),")
		print("}\n")

print("impl Inst {\n\tpub fn ends_bb(&self) -> bool {\n\t\tmatch self {")
for instr_name, modes in instr_modes.items():
	# Check if instruction is a conditional jump
	is_conditional_jump = instr_name in conditional_jumps

	# Check if instruction is an unconditional jump (JMP, JSR, RTS, RTI)
	is_unconditional_jump = instr_name in ["JMP", "JSR", "RTS", "RTI"]

	# Check if instruction is a return (RTS, RTI)
	is_return = instr_name in ["RTS", "RTI"]

	# Instructions that end basic blocks
	if is_conditional_jump or is_unconditional_jump or is_return:
		# For instructions with no parameters, we use just the name
		# For instructions with parameters, we use the pattern with ..
		if len(modes) == 1:
			m = modes[0]
			if m in ("Implied", "Accumulator"):
				print(f"\t\t\tInst::{instr_name} => true,")
			else:
				print(f"\t\t\tInst::{instr_name}(..) => true,")
		else:
			print(f"\t\t\tInst::{instr_name}(..) => true,")
	else:
		# For instructions that don't end basic blocks
		if len(modes) == 1:
			m = modes[0]
			if m in ("Implied", "Accumulator"):
				print(f"\t\t\tInst::{instr_name} => false,")
			else:
				print(f"\t\t\tInst::{instr_name}(..) => false,")
		else:
			print(f"\t\t\tInst::{instr_name}(..) => false,")
print("\t\t}\n\t}\n")

# Add method to get instruction length
print("\tpub fn len(&self) -> u8 {")
print("\t\tmatch self {")
for instr_name, modes in instr_modes.items():
	if len(modes) == 1:
		m = modes[0]
		instr_len = instruction_lengths[instr_name][m]
		if m in ("Implied", "Accumulator"):
			print(f"\t\t\tInst::{instr_name} => {instr_len},")
		else:
			print(f"\t\t\tInst::{instr_name}(..) => {instr_len},")
	else:
		# For instructions with multiple variants, we need to match each variant
		for mode in modes:
			instr_len = instruction_lengths[instr_name][mode]
			if mode in ("Implied", "Accumulator"):
				print(f"\t\t\tInst::{instr_name}({instr_name}::{mode}) => {instr_len},")
			else:
				print(f"\t\t\tInst::{instr_name}({instr_name}::{mode}(..)) => {instr_len},")
print("\t\t}\n\t}\n")

print("\tpub fn evaluate(&self, cpu: &mut Cpu) {")
for instr_name, moes in instr_modes.items():
	# Do stuff
print("\t\t}\n\t}\n}")
