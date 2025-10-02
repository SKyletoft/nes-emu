#include "interface.h"

// C-implementations of NES instructions

void cld(State *state) {
	state->cpu.p.D = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void cli(State *state) {
	state->cpu.p.I = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void clv(State *state) {
	state->cpu.p.V = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void cmp_impl(State *state, uint8_t val) {
	uint16_t res   = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.N = (res & 0x80) >> 7;
}

IMMEDIATE(cmp);
ZERO_PAGE(cmp);
ZERO_PAGE_X(cmp);
ABSOLUTE(cmp);
ABSOLUTE_X(cmp);
ABSOLUTE_Y(cmp);
INDIRECT_X(cmp);
INDIRECT_Y(cmp);

void cpx_impl(State *state, uint8_t val) {
	uint16_t res   = (uint16_t) state->cpu.x - (uint16_t) val;
	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.N = (res & 0x80) >> 7;
}

IMMEDIATE(cpx);
ZERO_PAGE(cpx);
ABSOLUTE(cpx);

void cpy_impl(State *state, uint8_t val) {
	uint16_t res   = (uint16_t) state->cpu.y - (uint16_t) val;
	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.N = (res & 0x80) >> 7;
}

IMMEDIATE(cpy);
ZERO_PAGE(cpy);
ABSOLUTE(cpy);

void dec_impl(State *state, uint8_t val) {
	val--;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

ZERO_PAGE(dec);
ZERO_PAGE_X(dec);
ABSOLUTE(dec);
ABSOLUTE_X(dec);

void dex(State *state) {
	state->cpu.x--;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void dey(State *state) {
	state->cpu.y--;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void eor_impl(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

IMMEDIATE(eor);
ZERO_PAGE(eor);
ZERO_PAGE_X(eor);
ABSOLUTE(eor);
ABSOLUTE_X(eor);
ABSOLUTE_Y(eor);
INDIRECT_X(eor);
INDIRECT_Y(eor);

void inc_impl(State *state, uint8_t val) {
	val++;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

ZERO_PAGE(inc);
ZERO_PAGE_X(inc);
ABSOLUTE(inc);
ABSOLUTE_X(inc);

void inx(State *state) {
	state->cpu.x++;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void iny(State *state) {
	state->cpu.y++;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

void jmp_absolute(State *state, uint16_t adr) {
	state->cpu.pc = adr;
	state_step_ppu_many(state, 4);
}

void jmp_indirect(State *state, uint16_t adr) {
	// Read the adress from memory at adr
	uint16_t low_byte  = state_get_mem(state, adr);
	uint16_t high_byte = state_get_mem(state, adr + 1);
	state->cpu.pc      = (uint16_t) ((high_byte << 8) | low_byte);
	state_step_ppu_many(state, 2);
}

void jsr(State *state, uint16_t adr) {
	// Push return adress (pc + 2) onto stack
	uint16_t return_adr = state->cpu.pc + 2;

	// Stack pointer is at cpu.s, but we need to adjust for the stack behaviour
	// The stack grows downwards from 0x1FF to 0x100
	uint8_t stack_ptr = state->cpu.s;

	// Push high byte first (stack grows downward)
	uint8_t high_byte = (return_adr >> 8) & 0xFF;
	state_set_mem(state, 0x100 + stack_ptr, high_byte);
	stack_ptr--;

	// Push low byte
	uint8_t low_byte = return_adr & 0xFF;
	state_set_mem(state, 0x100 + stack_ptr, low_byte);
	stack_ptr--;

	// Update stack pointer (stack grows downward)
	state->cpu.s = stack_ptr;

	// Jump to subroutine
	state->cpu.pc = adr;
	state_step_ppu_many(state, 2);
}
