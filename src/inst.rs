// Auto-generated NES CPU instruction set
pub enum Inst {
	ADC(ADC),
	AND(AND),
	ASL(ASL),
	BCC(i8),
	BCS(i8),
	BEQ(i8),
	BIT(BIT),
	BMI(i8),
	BNE(i8),
	BPL(i8),
	BRK,
	BVC(i8),
	BVS(i8),
	CLC,
	CLD,
	CLI,
	CLV,
	CMP(CMP),
	CPX(CPX),
	CPY(CPY),
	DEC(DEC),
	DEX,
	DEY,
	EOR(EOR),
	INC(INC),
	INX,
	INY,
	JMP(JMP),
	JSR(u8, u8),
	LDA(LDA),
	LDX(LDX),
	LDY(LDY),
	LSR(LSR),
	NOP,
	ORA(ORA),
	PHA,
	PHP,
	PLA,
	PLP,
	ROL(ROL),
	ROR(ROR),
	RTI,
	RTS,
	SBC(SBC),
	SEC,
	SED,
	SEI,
	STA(STA),
	STX(STX),
	STY(STY),
	TAX,
	TAY,
	TSX,
	TXA,
	TXS,
	TYA,
	LAX(LAX),
	SAX(SAX),
	DCP(DCP),
	ISC(ISC),
	RLA(RLA),
	RRA(RRA),
	SLO(SLO),
	SRE(SRE),
	ANC(u8),
	ALR(u8),
	ARR(u8),
	AXS(u8),
	LAS(u8, u8),
	TAS(u8, u8),
	SHY(u8, u8),
	SHX(u8, u8),
	AHX(AHX),
	NOPU(NOPU),
}

pub enum ADC {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum AND {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum ASL {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum BIT {
	ZeroPage(u8),
	Absolute(u8, u8),
}

pub enum CMP {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum CPX {
	Immediate(u8),
	ZeroPage(u8),
	Absolute(u8, u8),
}

pub enum CPY {
	Immediate(u8),
	ZeroPage(u8),
	Absolute(u8, u8),
}

pub enum DEC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum EOR {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum INC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum JMP {
	Absolute(u8, u8),
	Indirect(u8, u8),
}

pub enum LDA {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum LDX {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8, u8),
	AbsoluteY(u8, u8),
}

pub enum LDY {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum LSR {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum ORA {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum ROL {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum ROR {
	Accumulator,
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
}

pub enum SBC {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum STA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum STX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8, u8),
}

pub enum STY {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
}

pub enum LAX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum SAX {
	ZeroPage(u8),
	ZeroPageY(u8),
	Absolute(u8, u8),
	IndirectX(u8),
}

pub enum DCP {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum ISC {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum RLA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum RRA {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum SLO {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum SRE {
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
	IndirectX(u8),
	IndirectY(u8),
}

pub enum AHX {
	AbsoluteY(u8, u8),
	IndirectY(u8),
}

pub enum NOPU {
	Immediate(u8),
	ZeroPage(u8),
	ZeroPageX(u8),
	Absolute(u8, u8),
	AbsoluteX(u8, u8),
	AbsoluteY(u8, u8),
}

