#include "interface.h"

// C-implementations of NES instructions

void adc_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_zero_page_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_indirect_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_indirect_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void and_immediate(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_zero_page(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_zero_page_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute_y(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_indirect_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_indirect_y(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void asl_accumulator(State *state) {
	state->cpu.p.C = (state->cpu.a & 0x80) >> 7;
	state->cpu.a <<= 1;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void asl_zero_page(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_zero_page_x(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_absolute(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_absolute_x(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bcc(State *state, int8_t offset) {
	if (!state->cpu.p.C) {
		state->cpu.pc += offset;
	}
}

void bcs(State *state, int8_t offset) {
	if (state->cpu.p.C) {
		state->cpu.pc += offset;
	}
}

void beq(State *state, int8_t offset) {
	if (state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
}

void bit_zero_page(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bit_absolute(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bmi(State *state, int8_t offset) {
	if (state->cpu.p.N) {
		state->cpu.pc += offset;
	}
}

void bne(State *state, int8_t offset) {
	if (!state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
}

void bpl(State *state, int8_t offset) {
	if (!state->cpu.p.N) {
		state->cpu.pc += offset;
	}
}

void brk(State *state) {
	// BRK is a complex instruction that pushes PC+2 and status flags
	// This is a simplified version for demonstration
	state->cpu.pc++;
}

void bvc(State *state, int8_t offset) {
	if (!state->cpu.p.V) {
		state->cpu.pc += offset;
	}
}

void bvs(State *state, int8_t offset) {
	if (state->cpu.p.V) {
		state->cpu.pc += offset;
	}
}

void clc(State *state) {
	state->cpu.p.C = 0;
}
