#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

void sbc_impl(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);
	uint16_t a   = state->cpu.a;
	uint16_t val16 = val;

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ a) & (res ^ ~val16) & 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

IMMEDIATE(sbc);
ZERO_PAGE(sbc);
ZERO_PAGE_X(sbc);
ABSOLUTE(sbc);
ABSOLUTE_X(sbc);
ABSOLUTE_Y(sbc);
INDIRECT_X(sbc);
INDIRECT_Y(sbc);

void sec(State *state) {
	state->cpu.p.C = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void sed(State *state) {
	state->cpu.p.D = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void sei(State *state) {
	state->cpu.p.I = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void sta_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

void sta_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

void sta_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

void sta_absolute_x(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state->cpu.x + adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

void sta_absolute_y(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state->cpu.y + adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

void sta_indirect_x(State *state, uint8_t adr) {
	uint8_t zp    = (adr + state->cpu.x) & 0xFF;
	uint8_t lo    = state_get_mem(state, zp);
	uint8_t hi    = state_get_mem(state, (zp + 1) & 0xFF);
	uint16_t addr = (uint16_t) (lo | (hi << 8));
	state_set_mem(state, addr, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

void sta_indirect_y(State *state, uint8_t adr) {
	uint8_t lo    = state_get_mem(state, adr);
	uint8_t hi    = state_get_mem(state, (adr + 1) & 0xFF);
	uint16_t base = (uint16_t) (lo | (hi << 8));
	uint16_t addr = base + state->cpu.y;
	state_set_mem(state, addr, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

void stx_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.x);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

void stx_zero_page_y(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF, state->cpu.x);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

void stx_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.x);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

void sty_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.y);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

void sty_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF, state->cpu.y);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

void sty_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.y);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

void tax(State *state) {
	state->cpu.x   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void tay(State *state) {
	state->cpu.y   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void tsx(State *state) {
	state->cpu.x   = state->cpu.s;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void txa(State *state) {
	state->cpu.a   = state->cpu.x;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void txs(State *state) {
	state->cpu.s = state->cpu.x;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void tya(State *state) {
	state->cpu.a   = state->cpu.y;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void rti(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.s += 2;
	state->cpu.pc =
	    (uint16_t) (state_get_mem(state, (uint16_t) (state->cpu.s + 0x100 - 1))
	                | state_get_mem(state, (uint16_t) (state->cpu.s + 0x100)) << 8);
	state_step_ppu_many(state, 6);
}

void rts(State *state) {
	state->cpu.s += 2;
	state->cpu.pc =
	    (uint16_t) ((state_get_mem(state, (uint16_t) (state->cpu.s + 0x100 - 1))
	                 | state_get_mem(state, (uint16_t) (state->cpu.s + 0x100)) << 8)
	                + 1);
	state_step_ppu_many(state, 6);
}

void nop([[maybe_unused]] State *state) {
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}
