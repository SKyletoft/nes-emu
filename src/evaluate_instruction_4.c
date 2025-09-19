#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

void rti(State *state) {
	// Return from interrupt
}

void rts(State *state) {
	// Return from subroutine
}

void sbc_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_zero_page_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_indirect_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_indirect_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sec(State *state) {
	state->cpu.p.C = 1;
}

void sed(State *state) {
	state->cpu.p.D = 1;
}

void sei(State *state) {
	state->cpu.p.I = 1;
}

void sta_zero_page(State *state, uint8_t val) {
	// Store accumulator in memory
	state_set_mem(state, val, state->cpu.a);
}

void sta_zero_page_x(State *state, uint8_t val) {
	// Store accumulator in memory
	state_set_mem(state, val + state->cpu.x, state->cpu.a);
}

void sta_absolute(State *state, uint16_t adr) {
	// Store accumulator in memory
	state_set_mem(state, adr, state->cpu.a);
}

void sta_absolute_x(State *state, uint16_t adr) {
	// Store accumulator in memory
	state_set_mem(state, adr + state->cpu.x, state->cpu.a);
}

void sta_absolute_y(State *state, uint16_t adr) {
	// Store accumulator in memory
	state_set_mem(state, adr + state->cpu.y, state->cpu.a);
}

void sta_indirect_x(State *state, uint8_t val) {
	// Store accumulator in memory
	uint16_t addr = (uint16_t) val + (uint16_t) state->cpu.x;
	uint16_t ptr  = (uint16_t) ((uint16_t) state_get_mem(state, addr + 1) << 8
                                   | (uint16_t) state_get_mem(state, addr));
	state_set_mem(state, ptr, state->cpu.a);
}

void sta_indirect_y(State *state, uint8_t val) {
	// Store accumulator in memory
	uint16_t addr = (uint16_t) val;
	uint16_t ptr  = (uint16_t) ((uint16_t) state_get_mem(state, addr + 1) << 8
                                   | (uint16_t) state_get_mem(state, addr));
	state_set_mem(state, ptr + state->cpu.y, state->cpu.a);
}

void stx_zero_page(State *state, uint8_t val) {
	// Store X register in memory
	state_set_mem(state, val, state->cpu.x);
}

void stx_zero_page_y(State *state, uint8_t val) {
	// Store X register in memory
	state_set_mem(state, val + state->cpu.y, state->cpu.x);
}

void stx_absolute(State *state, uint8_t val) {
	// Store X register in memory
	state_set_mem(state, val, state->cpu.x);
}

void sty_zero_page(State *state, uint8_t val) {
	// Store Y register in memory
	state_set_mem(state, val, state->cpu.y);
}

void sty_zero_page_x(State *state, uint8_t val) {
	// Store Y register in memory
	state_set_mem(state, val + state->cpu.x, state->cpu.y);
}

void sty_absolute(State *state, uint8_t val) {
	// Store Y register in memory
	state_set_mem(state, val, state->cpu.y);
}

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
