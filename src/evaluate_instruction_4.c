#include "interface.h"
#include <stdint.h>
#include <stdio.h>

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
	state_step_ppu_many(state, 3);
};

void sta_absolute_y(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state->cpu.y + adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 3);
};

void sta_indirect_x(State *state, uint8_t adr) {
	uint8_t tmp   = state_get_mem(state, (uint16_t) (state->cpu.x + adr) & 0xFF);
	uint16_t adr2 = (uint16_t) (state_get_mem(state, (uint16_t) tmp)
				    | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);
	uint8_t val   = state_get_mem(state, adr2);
	state_set_mem(state, (uint16_t) val, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 2);
};

void sta_indirect_y(State *state, uint8_t adr) {
	uint8_t tmp   = state_get_mem(state, (uint16_t) (state->cpu.y + adr) & 0xFF);
	uint16_t adr2 = (uint16_t) (state_get_mem(state, (uint16_t) tmp)
				    | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);
	uint8_t val   = state_get_mem(state, adr2);
	state_set_mem(state, (uint16_t) val, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 2);
};

void stx_impl(State *state, uint8_t val) {
	puts("unimplemented");
}

ZERO_PAGE(stx);
ZERO_PAGE_Y(stx);
ABSOLUTE(stx);

void sty_impl(State *state, uint8_t val) {
	puts("unimplemented");
}

ZERO_PAGE(sty);
ZERO_PAGE_X(sty);
ABSOLUTE(sty);

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
	state->cpu.s += 1;
	state->cpu.pc = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state_step_ppu_many(state, 2);
}

void rts(State *state) {
	state->cpu.s += 1;
	state->cpu.pc = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state_step_ppu_many(state, 2);
}

void nop([[maybe_unused]] State *state) {
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}
