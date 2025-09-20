#include "interface.h"
#include <stdint.h>

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
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), state->cpu.a);
	state->cpu.s -= 1;
}

void php(State *state) {
	uint8_t val = state->cpu.p.raw | 0b00110000;
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), val);
	state->cpu.s -= 1;
}

void pla(State *state) {
	state->cpu.s += 1;
	state->cpu.a = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
}

void plp(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
}

void rol_impl(State *state, uint8_t *val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t) ((*val & 0x80) >> 7);
	*val           = (uint8_t) ((*val << 1) | carry);
	state->cpu.p.Z = (uint8_t) (0 == *val);
	state->cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(rol)
ZERO_PAGE_RMW(rol)
ZERO_PAGE_X_RMW(rol)
ABSOLUTE_RMW(rol)
ABSOLUTE_X_RMW(rol)

void ror_impl(State *state, uint8_t *val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t) (*val & 0x01);
	*val           = (uint8_t) ((carry << 7) | (*val >> 1));
	state->cpu.p.Z = (uint8_t) (0 == *val);
	state->cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(ror)
ZERO_PAGE_RMW(ror)
ZERO_PAGE_X_RMW(ror)
ABSOLUTE_RMW(ror)
ABSOLUTE_X_RMW(ror)
