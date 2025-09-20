#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

void sbc_impl(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

IMMEDIATE(sbc)
ZERO_PAGE(sbc)
ZERO_PAGE_X(sbc)
ABSOLUTE(sbc)
ABSOLUTE_X(sbc)
ABSOLUTE_Y(sbc)
INDIRECT_X(sbc)
INDIRECT_Y(sbc)

void sec(State *state) {
	state->cpu.p.C = 1;
}

void sed(State *state) {
	state->cpu.p.D = 1;
}

void sei(State *state) {
	state->cpu.p.I = 1;
}

void sta_impl(State *state, uint8_t val) {
	// Store accumulator in memory
	state_set_mem(state, val, state->cpu.a);
}

ZERO_PAGE(sta)
ZERO_PAGE_X(sta)
ABSOLUTE(sta)
ABSOLUTE_X(sta)
ABSOLUTE_Y(sta)
INDIRECT_X(sta)
INDIRECT_Y(sta)

void stx_impl(State *state, uint8_t val) {
	// Store X register in memory
	state_set_mem(state, val, state->cpu.x);
}

ZERO_PAGE(stx)
ZERO_PAGE_Y(stx)
ABSOLUTE(stx)

void sty_impl(State *state, uint8_t val) {
	// Store Y register in memory
	state_set_mem(state, val, state->cpu.y);
}

ZERO_PAGE(sty)
ZERO_PAGE_X(sty)
ABSOLUTE(sty)

void tax(State *state) {
	state->cpu.x   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void tay(State *state) {
	state->cpu.y   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void tsx(State *state) {
	state->cpu.x   = state->cpu.s;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void txa(State *state) {
	state->cpu.a   = state->cpu.x;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void txs(State *state) {
	state->cpu.s = state->cpu.x;
}

void tya(State *state) {
	state->cpu.a   = state->cpu.y;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void rti(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.s += 1;
	state->cpu.pc = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
}

void rts(State *state) {
	state->cpu.s += 1;
	state->cpu.pc = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
}

void nop([[maybe_unused]] State *state) {
}
