#include "interface.h"
#include <stdint.h>

// C-implementations of NES instructions

[[clang::always_inline]] static inline void adc_impl(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V =
	    ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

IMMEDIATE(adc);
ZERO_PAGE(adc);
ZERO_PAGE_X(adc);
ABSOLUTE(adc);
ABSOLUTE_X(adc);
ABSOLUTE_Y(adc);
INDIRECT_X(adc);
INDIRECT_Y(adc);

[[clang::always_inline]] static inline void and_impl(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

IMMEDIATE(and);
ZERO_PAGE(and);
ZERO_PAGE_X(and);
ABSOLUTE(and);
ABSOLUTE_X(and);
ABSOLUTE_Y(and);
INDIRECT_X(and);
INDIRECT_Y(and);

[[clang::always_inline]] static inline void asl_impl(State *state, uint8_t *val) {
	state->cpu.p.C = (*val & 0x80) >> 7;
	*val <<= 1;
	state->cpu.p.Z = 0 == *val;
	state->cpu.p.N = (*val & 0x80) >> 7;
}

ACCUMULATOR(asl);
ZERO_PAGE_RMW(asl);
ZERO_PAGE_X_RMW(asl);
ABSOLUTE_RMW(asl);
ABSOLUTE_X_RMW(asl);

STATIC_INLINE void bcc(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = !state->cpu.p.C;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void bcs(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = state->cpu.p.C;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void beq(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = state->cpu.p.Z;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

[[clang::always_inline]] static inline void bit_impl(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

ZERO_PAGE(bit);
ABSOLUTE(bit);

STATIC_INLINE void bmi(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = state->cpu.p.N;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void bne(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = !state->cpu.p.Z;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + taken + page_crossed;
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void bpl(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = !state->cpu.p.N;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void brk(State *state) {
	// BRK is a complex instruction that pushes PC+2 and status flags
	// This is a simplified version for demonstration
	state->cpu.pc++;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void bvc(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = !state->cpu.p.V;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void bvs(State *state, int8_t offset) {
	uint16_t old_pc   = state->cpu.pc;
	bool taken        = state->cpu.p.V;
	uint16_t new_pc   = old_pc + 2 + (taken ? (uint16_t) offset : 0);
	bool page_crossed = ((old_pc + 2) & 0xFF00) != (new_pc & 0xFF00);
	uint8_t cycles    = 2 + (taken ? 1 : 0) + (page_crossed ? 1 : 0);
	state->cpu.pc     = new_pc;
	state_step_ppu_many(state, cycles);
}

STATIC_INLINE void clc(State *state) {
	state->cpu.p.C = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void cld(State *state) {
	state->cpu.p.D = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void cli(State *state) {
	state->cpu.p.I = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void clv(State *state) {
	state->cpu.p.V = 0;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

[[clang::always_inline]] static inline void cmp_impl(State *state, uint8_t val) {
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

[[clang::always_inline]] static inline void cpx_impl(State *state, uint8_t val) {
	uint16_t res   = (uint16_t) state->cpu.x - (uint16_t) val;
	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.N = (res & 0x80) >> 7;
}

IMMEDIATE(cpx);
ZERO_PAGE(cpx);
ABSOLUTE(cpx);

[[clang::always_inline]] static inline void cpy_impl(State *state, uint8_t val) {
	uint16_t res   = (uint16_t) state->cpu.y - (uint16_t) val;
	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.N = (res & 0x80) >> 7;
}

IMMEDIATE(cpy);
ZERO_PAGE(cpy);
ABSOLUTE(cpy);

[[clang::always_inline]] static inline void dec_impl(State *state, uint8_t *val) {
	(*val)--;
	state->cpu.p.Z = 0 == *val;
	state->cpu.p.N = (*val & 0x80) >> 7;
}

ZERO_PAGE_RMW(dec);
ZERO_PAGE_X_RMW(dec);
ABSOLUTE_RMW(dec);
ABSOLUTE_X_RMW(dec);

STATIC_INLINE void dex(State *state) {
	state->cpu.x--;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void dey(State *state) {
	state->cpu.y--;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

[[clang::always_inline]] static inline void eor_impl(State *state, uint8_t val) {
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

[[clang::always_inline]] static inline void inc_impl(State *state, uint8_t *val) {
	(*val)++;
	state->cpu.p.Z = 0 == *val;
	state->cpu.p.N = (*val & 0x80) >> 7;
}

ZERO_PAGE_RMW(inc);
ZERO_PAGE_X_RMW(inc);
ABSOLUTE_RMW(inc);
ABSOLUTE_X_RMW(inc);

STATIC_INLINE void inx(State *state) {
	state->cpu.x++;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void iny(State *state) {
	state->cpu.y++;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void jmp_absolute(State *state, uint16_t adr) {
	state->cpu.pc = adr;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void jmp_indirect(State *state, uint16_t adr) {
	// Read the adress from memory at adr
	uint16_t low_byte  = state_get_mem(state, adr);
	uint16_t high_byte = state_get_mem(state, adr + 1);
	state->cpu.pc      = (uint16_t) ((high_byte << 8) | low_byte);
	state_step_ppu_many(state, 5);
}

STATIC_INLINE void jsr(State *state, uint16_t adr) {
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
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void lda_immediate(State *state, uint8_t val) {
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 2);
};

STATIC_INLINE void lda_zero_page(State *state, uint8_t offset) {
	uint8_t val    = state_get_mem(state, (uint16_t) offset);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void lda_zero_page_x(State *state, uint8_t offset) {
	uint8_t val  = state_get_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF);
	state->cpu.a = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void lda_absolute(State *state, uint16_t adr) {
	state_step_ppu(state);
	state_step_ppu(state);
	state_step_ppu(state);
	uint8_t val    = state_get_mem(state, adr);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu(state);
	state_check_interrupt(state);
};

STATIC_INLINE void lda_absolute_x(State *state, uint16_t adr) {
	uint16_t res   = state->cpu.x + adr;
	uint8_t val    = state_get_mem(state, res);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
	bool crossed = (res & 0xFF00) == (adr & 0xFF00);
	state_step_ppu_many(state, crossed ? 4 : 5);
};

STATIC_INLINE void lda_absolute_y(State *state, uint16_t adr) {
	uint16_t res   = state->cpu.y + adr;
	uint8_t val    = state_get_mem(state, res);
	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 3;
	bool crossed = (res & 0xFF00) == (adr & 0xFF00);
	state_step_ppu_many(state, crossed ? 4 : 5);
};

STATIC_INLINE void lda_indirect_x(State *state, uint8_t adr) {
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

STATIC_INLINE void lda_indirect_y(State *state, uint8_t adr) {
	uint16_t base = (uint16_t) (state_get_mem(state, (uint16_t) adr)
				    | (state_get_mem(state, (uint16_t) ((adr + 1) & 0xFF)) << 8));
	uint16_t adr2 = base + (uint16_t) state->cpu.y;
	uint8_t val   = state_get_mem(state, adr2);

	state->cpu.a   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.a);
	state->cpu.p.N = (uint8_t) ((state->cpu.a & 0x80) >> 7);
	state->cpu.pc += 2;

	bool page_crossed = (adr2 & 0xFF00) != (base & 0xFF00);
	state_step_ppu_many(state, page_crossed ? 6 : 5);
}

STATIC_INLINE void ldx_immediate(State *state, uint8_t val) {
	state_step_ppu_many(state, 2);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
}

STATIC_INLINE void ldx_zero_page(State *state, uint8_t offset) {
	uint8_t val    = state_get_mem(state, (uint16_t) offset);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void ldx_zero_page_y(State *state, uint8_t offset) {
	uint8_t val  = state_get_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF);
	state->cpu.x = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ldx_absolute(State *state, uint16_t adr) {
	state_step_ppu_many(state, 3);
	uint8_t val    = state_get_mem(state, adr);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void ldx_absolute_y(State *state, uint16_t adr) {
	uint8_t val    = state_get_mem(state, (uint16_t) state->cpu.y + adr);
	state->cpu.x   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.x);
	state->cpu.p.N = (uint8_t) ((state->cpu.x & 0x80) >> 7);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
}

[[clang::always_inline]] static inline void ldy_impl(State *state, uint8_t val) {
	state->cpu.y   = val;
	state->cpu.p.Z = (uint8_t) (0 == state->cpu.y);
	state->cpu.p.N = (uint8_t) ((state->cpu.y & 0x80) >> 7);
}

IMMEDIATE(ldy);
ZERO_PAGE(ldy);
ZERO_PAGE_X(ldy);
ABSOLUTE(ldy);
ABSOLUTE_X(ldy);

[[clang::always_inline]] static inline void lsr_impl(State *state, uint8_t *val) {
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

[[clang::always_inline]] static inline void ora_impl(State *state, uint8_t val) {
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

STATIC_INLINE void pha(State *state) {
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), state->cpu.a);
	state->cpu.s -= 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void php(State *state) {
	uint8_t val = state->cpu.p.raw | 0b00110000;
	state_set_mem(state, (uint16_t) (state->cpu.s + 0x100), val);
	state->cpu.s -= 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void pla(State *state) {
	state->cpu.s += 1;
	state->cpu.a = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.pc += 1;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void plp(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.pc += 1;
	state_step_ppu_many(state, 4);
}

[[clang::always_inline]] static inline void rol_impl(State *state, uint8_t *val) {
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

[[clang::always_inline]] static inline void ror_impl(State *state, uint8_t *val) {
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

[[clang::always_inline]] static inline void sbc_impl(State *state, uint8_t val) {
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

STATIC_INLINE void sec(State *state) {
	state->cpu.p.C = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sed(State *state) {
	state->cpu.p.D = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sei(State *state) {
	state->cpu.p.I = 1;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sta_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void sta_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sta_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sta_absolute_x(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state->cpu.x + adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

STATIC_INLINE void sta_absolute_y(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state->cpu.y + adr, state->cpu.a);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

STATIC_INLINE void sta_indirect_x(State *state, uint8_t adr) {
	uint8_t zp    = (adr + state->cpu.x) & 0xFF;
	uint8_t lo    = state_get_mem(state, zp);
	uint8_t hi    = state_get_mem(state, (zp + 1) & 0xFF);
	uint16_t addr = (uint16_t) (lo | (hi << 8));
	state_set_mem(state, addr, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void sta_indirect_y(State *state, uint8_t adr) {
	uint8_t lo    = state_get_mem(state, adr);
	uint8_t hi    = state_get_mem(state, (adr + 1) & 0xFF);
	uint16_t base = (uint16_t) (lo | (hi << 8));
	uint16_t addr = base + state->cpu.y;
	state_set_mem(state, addr, state->cpu.a);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void stx_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.x);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void stx_zero_page_y(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF, state->cpu.x);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void stx_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.x);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sty_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state->cpu.y);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void sty_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF, state->cpu.y);
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sty_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state->cpu.y);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void tax(State *state) {
	state->cpu.x   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tay(State *state) {
	state->cpu.y   = state->cpu.a;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tsx(State *state) {
	state->cpu.x   = state->cpu.s;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void txa(State *state) {
	state->cpu.a   = state->cpu.x;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void txs(State *state) {
	state->cpu.s = state->cpu.x;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tya(State *state) {
	state->cpu.a   = state->cpu.y;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void rti(State *state) {
	state->cpu.s += 1;
	state->cpu.p.raw = state_get_mem(state, (uint16_t) (state->cpu.s + 0x100));
	state->cpu.s += 2;
	state->cpu.pc =
	    (uint16_t) (state_get_mem(state, (uint16_t) (state->cpu.s + 0x100 - 1))
			| state_get_mem(state, (uint16_t) (state->cpu.s + 0x100)) << 8);
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void rts(State *state) {
	state->cpu.s += 2;
	state->cpu.pc =
	    (uint16_t) ((state_get_mem(state, (uint16_t) (state->cpu.s + 0x100 - 1))
			 | state_get_mem(state, (uint16_t) (state->cpu.s + 0x100)) << 8)
			+ 1);
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void nop(State *state) {
	state->cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void skb(State *state) {
	state->cpu.pc += 2;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void ign(State *state, uint16_t) {
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_direct(State *state, uint8_t) {
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_direct_x(State *state, uint8_t) {
	state->cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_absolute_x(State *state, uint16_t adr) {
	uint16_t actual_adr = (uint16_t) state->cpu.x + adr;
	bool page_crossed   = state->cpu.x + (adr & 0xFF) > 0xFF;
	(void) state_get_mem(state, actual_adr);
	state->cpu.pc += 3;
	state_step_ppu_many(state, 4 + page_crossed);
}

STATIC_INLINE void lax_immediate(State *state, uint8_t) {
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_zero_page(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_zero_page_y(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_absolute(State *state, uint16_t val) {
	state->cpu.a = (uint8_t) val;
	state->cpu.x = (uint8_t) val;
	// Update flags
	state->cpu.p.Z = ((uint8_t) val == 0);
	state->cpu.p.N = (((uint8_t) val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_absolute_y(State *state, uint16_t val) {
	state->cpu.a = (uint8_t) val;
	state->cpu.x = (uint8_t) val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_indirect_x(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void lax_indirect_y(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void sax_zero_page(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

STATIC_INLINE void sax_zero_page_y(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

STATIC_INLINE void sax_absolute(State *state, uint16_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, (uint8_t) val, result);
	state->cpu.pc += 2;
}

STATIC_INLINE void sax_indirect_x(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_zero_page(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_zero_page_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val - 1;
	state_set_mem(state, (uint8_t) val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute_x(State *state, uint16_t val) {
	uint8_t result = (uint8_t)val - 1;
	state_set_mem(state, (uint8_t)val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute_y(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val - 1;
	state_set_mem(state, (uint8_t) val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_indirect_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void dcp_indirect_y(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_zero_page(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_zero_page_x(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_absolute(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, (uint8_t) val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_absolute_x(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_absolute_y(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_indirect_x(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void isc_indirect_y(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state->cpu.a - result - (1 - state->cpu.p.C);
	state->cpu.p.C = (temp <= state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.a   = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_zero_page(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_zero_page_x(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_absolute(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_absolute_x(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_absolute_y(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_indirect_x(State *state, uint8_t val) {
	// Rotate left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rla_indirect_y(State *state, uint8_t val) {
	// Rotate left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state->cpu.p.C);
	state->cpu.p.C = carry;

	// AND with accumulator
	result &= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_zero_page(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_zero_page_x(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_absolute(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_absolute_x(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_absolute_y(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_indirect_x(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void rra_indirect_y(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state->cpu.p.C << 7));
	state->cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state->cpu.a + result + state->cpu.p.C;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = temp;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_zero_page(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_zero_page_x(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_absolute(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_absolute_x(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_absolute_y(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_indirect_x(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void slo_indirect_y(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state->cpu.p.C = carry;

	// OR with accumulator
	result |= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_zero_page(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_zero_page_x(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_absolute(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_absolute_x(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_absolute_y(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_indirect_x(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void sre_indirect_y(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state->cpu.p.C = carry;

	// XOR with accumulator
	result ^= state->cpu.a;

	// Update flags
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state->cpu.a = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void anc_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state->cpu.a &= val;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);

	// Set carry flag to bit 7 of accumulator
	state->cpu.p.C = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void alr_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state->cpu.a &= val;

	// Shift right
	uint8_t carry = (state->cpu.a & 0x01) ? 0x80 : 0;
	state->cpu.a >>= 1;
	state->cpu.p.C = carry;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void arr_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state->cpu.a &= val;

	// Shift right with carry
	uint8_t carry = (state->cpu.a & 0x01) ? 0x80 : 0;
	state->cpu.a >>= 1;
	state->cpu.a |= ((uint8_t) state->cpu.p.C << 7);
	state->cpu.p.C = carry;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void axs_immediate(State *state, uint8_t val) {
	// AND accumulator with X register
	uint8_t temp = state->cpu.a & state->cpu.x;

	// Subtract with borrow
	uint8_t result = temp - val - (1 - state->cpu.p.C);
	state->cpu.p.C = (result <= temp);
	state->cpu.p.Z = (result == 0);
	state->cpu.p.N = ((result & 0x80) != 0);

	// Store result in X register
	state->cpu.x = result;
	state->cpu.pc += 2;
}

STATIC_INLINE void las_immediate(State *state, uint8_t val) {
	// AND with accumulator and store in A, X, and S
	state->cpu.a &= val;
	state->cpu.x = state->cpu.a;
	state->cpu.s = state->cpu.a;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

STATIC_INLINE void tas_immediate(State *state, uint8_t val) {
	// AND accumulator with X register and store in S
	uint8_t temp = state->cpu.a & state->cpu.x;
	state->cpu.s = temp;

	// Store S in memory
	state_set_mem(state, val, temp);
	state->cpu.pc += 2;
}

STATIC_INLINE void tas_absolute_y(State *state, uint16_t) {
	state->cpu.pc += 3;
}

STATIC_INLINE void shy_immediate(State *state, uint8_t val) {
	// Store Y register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store Y register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.y);
	state->cpu.pc += 2;
}

STATIC_INLINE void shy_absolute_x(State *state, uint16_t) {
	state->cpu.pc += 3;
}

STATIC_INLINE void shx_immediate(State *state, uint8_t val) {
	// Store X register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store X register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.x);
	state->cpu.pc += 2;
}

STATIC_INLINE void shx_absolute_y(State *state, uint16_t) {
	state->cpu.pc += 3;
}

STATIC_INLINE void ahx_absolute_y(State *state, uint16_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = (uint8_t) val;
	uint8_t adr_high = ((uint8_t) val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.a & state->cpu.x);
	state->cpu.pc += 2;
}

STATIC_INLINE void ahx_indirect_y(State *state, uint8_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.a & state->cpu.x);
	state->cpu.pc += 2;
}

STATIC_INLINE void stp(State *state) {
	state->cpu.pc += 1;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void xaa_immediate(State *state, uint8_t) {
	state->cpu.pc += 2;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void las_absolute_y(State *state, uint16_t) {
	state->cpu.pc += 2;
	state_step_ppu_many(state, 1);
}
