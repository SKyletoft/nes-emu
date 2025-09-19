#include "interface.h"

// C-implementations of NES instructions

void lda_immediate(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_zero_page(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_zero_page_x(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_absolute(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_absolute_x(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_absolute_y(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_indirect_x(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lda_indirect_y(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ldx_immediate(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void ldx_zero_page(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void ldx_zero_page_y(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void ldx_absolute(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void ldx_absolute_y(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void ldy_immediate(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void ldy_zero_page(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void ldy_zero_page_x(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void ldy_absolute(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void ldy_absolute_x(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void lsr_accumulator(State *state) {
	state->cpu.p.C = state->cpu.a & 0x01;
	state->cpu.a >>= 1;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void lsr_zero_page(State *state, uint8_t val) {
	state->cpu.p.C = val & 0x01;
	val >>= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void lsr_zero_page_x(State *state, uint8_t val) {
	state->cpu.p.C = val & 0x01;
	val >>= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void lsr_absolute(State *state, uint8_t val) {
	state->cpu.p.C = val & 0x01;
	val >>= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void lsr_absolute_x(State *state, uint8_t val) {
	state->cpu.p.C = val & 0x01;
	val >>= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void nop(State *state) {
	// No operation
}

void ora_immediate(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_zero_page(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_zero_page_x(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_absolute(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_absolute_x(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_absolute_y(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_indirect_x(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void ora_indirect_y(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void pha(State *state) {
	// Push accumulator onto stack
}

void php(State *state) {
	// Push processor status onto stack
}

void pla(State *state) {
	// Pull accumulator from stack
}

void plp(State *state) {
	// Pull processor status from stack
}

void rol_accumulator(State *state) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = (state->cpu.a & 0x80) >> 7;
	state->cpu.a        = ((state->cpu.a << 1) | carry) != 0;
	state->cpu.p.Z      = 0 == state->cpu.a;
	state->cpu.p.N      = (state->cpu.a & 0x80) >> 7;
}

void rol_zero_page(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void rol_zero_page_x(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void rol_absolute(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void rol_absolute_x(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void ror_accumulator(State *state) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = state->cpu.a & 0x01;
	state->cpu.a        = ((carry << 7) | (state->cpu.a >> 1)) != 0;
	state->cpu.p.Z      = 0 == state->cpu.a;
	state->cpu.p.N      = (state->cpu.a & 0x80) >> 7;
}

void ror_zero_page(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void ror_zero_page_x(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void ror_absolute(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}

void ror_absolute_x(State *state, uint8_t val) {
	uint8_t carry = state->cpu.p.C;
	state->cpu.p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	state->cpu.p.Z      = 0 == val;
	state->cpu.p.N      = (val & 0x80) >> 7;
}
