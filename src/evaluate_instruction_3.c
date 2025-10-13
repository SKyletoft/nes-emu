#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

void lda_immediate(State *state, uint8_t val) {
	state_step_ppu_many(state, 2);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
};

void lda_zero_page(State *state, uint8_t offset) {
	uint8_t val    = state_get_mem(state, (uint16_t) offset);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

void lda_zero_page_x(State *state, uint8_t offset) {
	uint8_t val  = state_get_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF);
	state->cpu.a = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

void lda_absolute(State *state, uint16_t adr) {
	state_step_ppu_many(state, 3);
	uint8_t val = state_get_mem(state, adr);
	state_step_ppu_many(state, 1);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
};

void lda_absolute_x(State *state, uint16_t adr) {
	uint8_t val    = state_get_mem(state, (uint16_t) state->cpu.x + adr);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

void lda_absolute_y(State *state, uint16_t adr) {
	uint8_t val    = state_get_mem(state, (uint16_t) state->cpu.y + adr);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

void lda_indirect_x(State *state, uint8_t adr) {
	uint8_t tmp    = state_get_mem(state, (uint16_t) (state->cpu.x + adr) & 0xFF);
	uint16_t adr2  = (uint16_t) (state_get_mem(state, (uint16_t) tmp)
                                    | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);
	uint8_t val    = state_get_mem(state, adr2);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 6);
};

void lda_indirect_y(State *state, uint8_t adr) {
	uint8_t tmp    = state_get_mem(state, (uint16_t) (state->cpu.y + adr) & 0xFF);
	uint16_t adr2  = (uint16_t) (state_get_mem(state, (uint16_t) tmp)
                                    | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);
	uint8_t val    = state_get_mem(state, adr2);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;

	bool page_crossed = (adr2 & 0xFF00) != (tmp & 0xFF00);
	state_step_ppu_many(state, page_crossed ? 6 : 5);
};

void ldx_immediate(State *state, uint8_t val) {
	state_step_ppu_many(state, 2);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
}

void ldx_zero_page(State *state, uint8_t offset) {
	uint8_t val    = state_get_mem(state, (uint16_t) offset);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
}

void ldx_zero_page_y(State *state, uint8_t offset) {
	uint8_t val  = state_get_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF);
	state->cpu.x = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

void ldx_absolute(State *state, uint16_t adr) {
	state_step_ppu_many(state, 3);
	uint8_t val = state_get_mem(state, adr);
	state_step_ppu_many(state, 1);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 3;
}

void ldx_absolute_y(State *state, uint16_t adr) {
	uint8_t val    = state_get_mem(state, (uint16_t) state->cpu.y + adr);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
}

void ldy_impl(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.y);
	state->cpu.p.N = (uint8_t) ((state->cpu.y & 0x80) >> 7);
}

IMMEDIATE(ldy);
ZERO_PAGE(ldy);
ZERO_PAGE_X(ldy);
ABSOLUTE(ldy);
ABSOLUTE_X(ldy);

void lsr_impl(State *state, uint8_t *val) {
	state->cpu.p.C = (uint8_t) (*val & 0x01);
	*val >>= 1;
	state->cpu.p.Z = (uint8_t) (0 == *val);
	state->cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(lsr);
ZERO_PAGE_RMW(lsr);
ZERO_PAGE_X_RMW(lsr);
ABSOLUTE_RMW(lsr);
ABSOLUTE_X_RMW(lsr);

void ora_impl(State *state, uint8_t val) {
	state->cpu.a |= val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
}

IMMEDIATE(ora);
ZERO_PAGE(ora);
ZERO_PAGE_X(ora);
ABSOLUTE(ora);
ABSOLUTE_X(ora);
ABSOLUTE_Y(ora);
INDIRECT_X(ora);
INDIRECT_Y(ora);

void pha(State *state) {
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), state->cpu.a);
	state->cpu.s -= 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

void php(State *state) {
	uint8_t val = state->cpu.p.raw | 0b00110000;
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), val);
	state->cpu.s -= 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

void pla(State *state) {
	state->cpu.s += 1;
	state->cpu.a = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.pc += 1;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
	state_step_ppu_many(state, 4);
}

void plp(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.pc += 1;
	state_step_ppu_many(state, 4);
}

void rol_impl(State *state, uint8_t *val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t) ((*val & 0x80) >> 7);
	*val           = (uint8_t) ((*val << 1) | carry);
	state->cpu.p.Z = (uint8_t) (0 == *val);
	state->cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(rol);
ZERO_PAGE_RMW(rol);
ZERO_PAGE_X_RMW(rol);
ABSOLUTE_RMW(rol);
ABSOLUTE_X_RMW(rol);

void ror_impl(State *state, uint8_t *val) {
	uint8_t carry  = state->cpu.p.C;
	state->cpu.p.C = (uint8_t) (*val & 0x01);
	*val           = (uint8_t) ((carry << 7) | (*val >> 1));
	state->cpu.p.Z = (uint8_t) (0 == *val);
	state->cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(ror);
ZERO_PAGE_RMW(ror);
ZERO_PAGE_X_RMW(ror);
ABSOLUTE_RMW(ror);
ABSOLUTE_X_RMW(ror);
