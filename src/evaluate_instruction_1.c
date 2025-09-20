#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

void adc_impl(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

IMMEDIATE(adc)
ZERO_PAGE(adc)
ZERO_PAGE_X(adc)
ABSOLUTE(adc)
ABSOLUTE_X(adc)
ABSOLUTE_Y(adc)
INDIRECT_X(adc)
INDIRECT_Y(adc)

void and_impl(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

IMMEDIATE(and)
ZERO_PAGE(and)
ZERO_PAGE_X(and)
ABSOLUTE(and)
ABSOLUTE_X(and)
ABSOLUTE_Y(and)
INDIRECT_X(and)
INDIRECT_Y(and)

void asl_impl(State *state, uint8_t *val) {
	state->cpu.p.C = (*val & 0x80) >> 7;
	*val <<= 1;
	state->cpu.p.Z = 0 == *val;
	state->cpu.p.N = (*val & 0x80) >> 7;
}

ACCUMULATOR(asl)
ZERO_PAGE_RMW(asl)
ZERO_PAGE_X_RMW(asl)
ABSOLUTE_RMW(asl)
ABSOLUTE_X_RMW(asl)

void bcc(State *state, int8_t offset) {
	if (!state->cpu.p.C) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void bcs(State *state, int8_t offset) {
	if (state->cpu.p.C) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void beq(State *state, int8_t offset) {
	if (state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void bit_impl(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

ZERO_PAGE(bit)
ABSOLUTE(bit)

void bmi(State *state, int8_t offset) {
	if (state->cpu.p.N) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void bne(State *state, int8_t offset) {
	if (!state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void bpl(State *state, int8_t offset) {
	if (!state->cpu.p.N) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
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
	state->cpu.pc += 2;
}

void bvs(State *state, int8_t offset) {
	if (state->cpu.p.V) {
		state->cpu.pc += offset;
	}
	state->cpu.pc += 2;
}

void clc(State *state) {
	state->cpu.p.C = 0;
	state->cpu.pc += 1;
}
