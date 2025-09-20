#include "interface.h"

// C-implementations of NES instructions

void lda_impl(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
}

IMMEDIATE(lda)
ZERO_PAGE(lda)
ZERO_PAGE_X(lda)
ABSOLUTE(lda)
ABSOLUTE_X(lda)
ABSOLUTE_Y(lda)
INDIRECT_X(lda)
INDIRECT_Y(lda)

void ldx_impl(State *state, uint8_t val) {
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
}

IMMEDIATE(ldx)
ZERO_PAGE(ldx)
ZERO_PAGE_Y(ldx)
ABSOLUTE(ldx)
ABSOLUTE_Y(ldx)

void ldy_impl(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.y);
	state->cpu.p.N = (uint8_t) ((state->cpu.y & 0x80) >> 7);
}

IMMEDIATE(ldy)
ZERO_PAGE(ldy)
ZERO_PAGE_X(ldy)
ABSOLUTE(ldy)
ABSOLUTE_X(ldy)

void lsr_accumulator(State *state) {
	state->cpu.p.C = (uint8_t) (state->cpu.a & 0x01);
	state->cpu.a >>= 1;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
}

void lsr_impl(State *state, uint8_t val) {
	state->cpu.p.C = (uint8_t) (val & 0x01);
	val >>= 1;
	state->cpu.p.Z = (uint8_t) (0 == val);
	state->cpu.p.N = (uint8_t) ((val & 0x80) >> 7);
}

ZERO_PAGE(lsr)
ZERO_PAGE_X(lsr)
ABSOLUTE(lsr)
ABSOLUTE_X(lsr)

void ora_impl(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
}

IMMEDIATE(ora)
ZERO_PAGE(ora)
ZERO_PAGE_X(ora)
ABSOLUTE(ora)
ABSOLUTE_X(ora)
ABSOLUTE_Y(ora)
INDIRECT_X(ora)
INDIRECT_Y(ora)

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
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t)((state->cpu.a & 0x80) >> 7);
	state->cpu.a   = (uint8_t)((state->cpu.a << 1) | carry);
	state->cpu.p.Z = (uint8_t)(0 == state->cpu.a);
	state->cpu.p.N = (uint8_t)((state->cpu.a & 0x80) >> 7);
}

void rol_impl(State *state, uint8_t val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t)((val & 0x80) >> 7);
	val            = (uint8_t)((val << 1) | carry);
	state->cpu.p.Z = (uint8_t)(0 == val);
	state->cpu.p.N = (uint8_t)((val & 0x80) >> 7);
}

ZERO_PAGE(rol)
ZERO_PAGE_X(rol)
ABSOLUTE(rol)
ABSOLUTE_X(rol)

void ror_accumulator(State *state) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t)(state->cpu.a & 0x01);
	state->cpu.a   = (uint8_t)((carry << 7) | (state->cpu.a >> 1));
	state->cpu.p.Z = (uint8_t)(0 == state->cpu.a);
	state->cpu.p.N = (uint8_t)((state->cpu.a & 0x80) >> 7);
}

void ror_impl(State *state, uint8_t val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t)(val & 0x01);
	val            = (uint8_t)((carry << 7) | (val >> 1));
	state->cpu.p.Z = (uint8_t)(0 == val);
	state->cpu.p.N = (uint8_t)((val & 0x80) >> 7);
}

ZERO_PAGE(ror)
ZERO_PAGE_X(ror)
ABSOLUTE(ror)
ABSOLUTE_X(ror)
