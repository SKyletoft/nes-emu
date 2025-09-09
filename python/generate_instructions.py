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

conditional_jumps = {
	"BCC", "BCS", "BEQ", "BMI", "BNE", "BPL", "BVC", "BVS"
}

print("#![allow(unused)]")
print()
print("// Auto-generated NES CPU instruction set")
print("pub enum Inst {")
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
		print(f"pub enum {instr} {{")
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

print("impl Inst {\n\tfn ends_bb(&self) -> bool {\n\t\tmatch self {")
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
print("\t\t}\n\t}\n}")
