#include <stdint.h>

typedef struct {
	uint8_t C       : 1;
	uint8_t Z       : 1;
	uint8_t I       : 1;
	uint8_t D       : 1;
	uint8_t B       : 1;
	uint8_t _unused : 1;
	uint8_t V       : 1;
	uint8_t N       : 1;
} P;

typedef struct {
	uint8_t a;
	uint8_t x;
	uint8_t y;
	uint8_t s;
	P p;
	uint16_t pc;
} Cpu;

typedef struct {
	Cpu cpu;
	/* Mapper */ void *rom;
	uint8_t ram[2048];
} State;

uint8_t state_get_mem(State const *state, uint16_t adr);
void state_set_mem(State const *state, uint16_t adr, uint8_t val);

// C-implementations of NES instructions

void adc_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_zero_page_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_absolute_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_indirect_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void adc_indirect_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a + (uint16_t) state->cpu.p.C + (uint16_t) val;

	state->cpu.p.C = res > 255;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void and_immediate(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_zero_page(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_zero_page_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_absolute_y(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_indirect_x(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void and_indirect_y(State *state, uint8_t val) {
	state->cpu.a &= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void asl_accumulator(State *state) {
	state->cpu.p.C = (state->cpu.a & 0x80) >> 7;
	state->cpu.a <<= 1;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void asl_zero_page(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_zero_page_x(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_absolute(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void asl_absolute_x(State *state, uint8_t val) {
	state->cpu.p.C = (val & 0x80) >> 7;
	val <<= 1;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bcc(State *state, int8_t offset) {
	if (!state->cpu.p.C) {
		state->cpu.pc += offset;
	}
}

void bcs(State *state, int8_t offset) {
	if (state->cpu.p.C) {
		state->cpu.pc += offset;
	}
}

void beq(State *state, int8_t offset) {
	if (state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
}

void bit_zero_page(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bit_absolute(State *state, uint8_t val) {
	state->cpu.p.Z = 0 == (state->cpu.a & val);
	state->cpu.p.V = (val & 0x40) >> 6;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void bmi(State *state, int8_t offset) {
	if (state->cpu.p.N) {
		state->cpu.pc += offset;
	}
}

void bne(State *state, int8_t offset) {
	if (!state->cpu.p.Z) {
		state->cpu.pc += offset;
	}
}

void bpl(State *state, int8_t offset) {
	if (!state->cpu.p.N) {
		state->cpu.pc += offset;
	}
}

void brk(State *state) {
	// BRK is a complex instruction that pushes PC+2 and status flags
	// This is a simplified version for demonstration
	state->cpu.pc++;
}

void bvc(State *state, int8_t offset) {
	if (!state->cpu.p.V) {
		state->cpu.pc += offset;
	}
}

void bvs(State *state, int8_t offset) {
	if (state->cpu.p.V) {
		state->cpu.pc += offset;
	}
}

void clc(State *state) {
	state->cpu.p.C = 0;
}

void cld(State *state) {
	state->cpu.p.D = 0;
}

void cli(State *state) {
	state->cpu.p.I = 0;
}

void clv(State *state) {
	state->cpu.p.V = 0;
}

void cmp_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_zero_page_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_absolute_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_absolute_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_indirect_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cmp_indirect_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpx_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.x - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpx_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.x - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpx_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.x - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpy_immediate(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.y - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpy_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.y - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void cpy_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.y - (uint16_t) val;
	state->cpu.p.C     = res < 256;
	state->cpu.p.Z     = 0 == (uint8_t) res;
	state->cpu.p.N     = (res & 0x80) >> 7;
}

void dec_zero_page(State *state, uint8_t val) {
	val--;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void dec_zero_page_x(State *state, uint8_t val) {
	val--;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void dec_absolute(State *state, uint8_t val) {
	val--;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void dec_absolute_x(State *state, uint8_t val) {
	val--;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void dex(State *state) {
	state->cpu.x--;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void dey(State *state) {
	state->cpu.y--;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void eor_immediate(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_zero_page(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_zero_page_x(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_absolute(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_absolute_x(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_absolute_y(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_indirect_x(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void eor_indirect_y(State *state, uint8_t val) {
	state->cpu.a ^= val;
	state->cpu.p.Z = 0 == state->cpu.a;
	state->cpu.p.N = (state->cpu.a & 0x80) >> 7;
}

void inc_zero_page(State *state, uint8_t val) {
	val++;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void inc_zero_page_x(State *state, uint8_t val) {
	val++;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void inc_absolute(State *state, uint8_t val) {
	val++;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void inc_absolute_x(State *state, uint8_t val) {
	val++;
	state->cpu.p.Z = 0 == val;
	state->cpu.p.N = (val & 0x80) >> 7;
}

void inx(State *state) {
	state->cpu.x++;
	state->cpu.p.Z = 0 == state->cpu.x;
	state->cpu.p.N = (state->cpu.x & 0x80) >> 7;
}

void iny(State *state) {
	state->cpu.y++;
	state->cpu.p.Z = 0 == state->cpu.y;
	state->cpu.p.N = (state->cpu.y & 0x80) >> 7;
}

void jmp_absolute(State *state, uint16_t addr) {
	state->cpu.pc = addr;
}

void jmp_indirect(State *state, uint16_t addr) {
	state->cpu.pc = addr;
}

void jsr(State *state, uint16_t addr) {
	// Push return address (pc + 2)
	state->cpu.pc = addr;
}

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
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_zero_page(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_zero_page_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_absolute_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_indirect_x(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	state->cpu.p.N = (res & 0x80) >> 7;
	state->cpu.a   = (uint8_t) res;
}

void sbc_indirect_y(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state->cpu.a - (uint16_t) val - (uint16_t) (1 - state->cpu.p.C);

	state->cpu.p.C = res < 256;
	state->cpu.p.Z = 0 == (uint8_t) res;
	state->cpu.p.V = ((res ^ (uint16_t) state->cpu.a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
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
}

void sta_zero_page_x(State *state, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute(State *state, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute_x(State *state, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute_y(State *state, uint8_t val) {
	// Store accumulator in memory
}

void sta_indirect_x(State *state, uint8_t val) {
	// Store accumulator in memory
}

void sta_indirect_y(State *state, uint8_t val) {
	// Store accumulator in memory
}

void stx_zero_page(State *state, uint8_t val) {
	// Store X register in memory
}

void stx_zero_page_y(State *state, uint8_t val) {
	// Store X register in memory
}

void stx_absolute(State *state, uint8_t val) {
	// Store X register in memory
}

void sty_zero_page(State *state, uint8_t val) {
	// Store Y register in memory
}

void sty_zero_page_x(State *state, uint8_t val) {
	// Store Y register in memory
}

void sty_absolute(State *state, uint8_t val) {
	// Store Y register in memory
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

void lax_zero_page(State *state, uint8_t val) {}
void lax_zero_page_y(State *state, uint8_t val) {}
void lax_absolute(State *state, uint8_t val) {}
void lax_absolute_y(State *state, uint8_t val) {}
void lax_indirect_x(State *state, uint8_t val) {}
void lax_indirect_y(State *state, uint8_t val) {}
void sax_zero_page(State *state, uint8_t val) {}
void sax_zero_page_y(State *state, uint8_t val) {}
void sax_absolute(State *state, uint8_t val) {}
void sax_indirect_x(State *state, uint8_t val) {}
void dcp_zero_page(State *state, uint8_t val) {}
void dcp_zero_page_x(State *state, uint8_t val) {}
void dcp_absolute(State *state, uint8_t val) {}
void dcp_absolute_x(State *state, uint8_t val) {}
void dcp_absolute_y(State *state, uint8_t val) {}
void dcp_indirect_x(State *state, uint8_t val) {}
void dcp_indirect_y(State *state, uint8_t val) {}
void isc_zero_page(State *state, uint8_t val) {}
void isc_zero_page_x(State *state, uint8_t val) {}
void isc_absolute(State *state, uint8_t val) {}
void isc_absolute_x(State *state, uint8_t val) {}
void isc_absolute_y(State *state, uint8_t val) {}
void isc_indirect_x(State *state, uint8_t val) {}
void isc_indirect_y(State *state, uint8_t val) {}
void rla_zero_page(State *state, uint8_t val) {}
void rla_zero_page_x(State *state, uint8_t val) {}
void rla_absolute(State *state, uint8_t val) {}
void rla_absolute_x(State *state, uint8_t val) {}
void rla_absolute_y(State *state, uint8_t val) {}
void rla_indirect_x(State *state, uint8_t val) {}
void rla_indirect_y(State *state, uint8_t val) {}
void rra_zero_page(State *state, uint8_t val) {}
void rra_zero_page_x(State *state, uint8_t val) {}
void rra_absolute(State *state, uint8_t val) {}
void rra_absolute_x(State *state, uint8_t val) {}
void rra_absolute_y(State *state, uint8_t val) {}
void rra_indirect_x(State *state, uint8_t val) {}
void rra_indirect_y(State *state, uint8_t val) {}
void slo_zero_page(State *state, uint8_t val) {}
void slo_zero_page_x(State *state, uint8_t val) {}
void slo_absolute(State *state, uint8_t val) {}
void slo_absolute_x(State *state, uint8_t val) {}
void slo_absolute_y(State *state, uint8_t val) {}
void slo_indirect_x(State *state, uint8_t val) {}
void slo_indirect_y(State *state, uint8_t val) {}
void sre_zero_page(State *state, uint8_t val) {}
void sre_zero_page_x(State *state, uint8_t val) {}
void sre_absolute(State *state, uint8_t val) {}
void sre_absolute_x(State *state, uint8_t val) {}
void sre_absolute_y(State *state, uint8_t val) {}
void sre_indirect_x(State *state, uint8_t val) {}
void sre_indirect_y(State *state, uint8_t val) {}
void anc(State *state, uint8_t val) {}
void alr(State *state, uint8_t val) {}
void arr(State *state, uint8_t val) {}
void axs(State *state, uint8_t val) {}
void las(State *state, uint8_t val) {}
void tas(State *state, uint8_t val) {}
void shy(State *state, uint8_t val) {}
void shx(State *state, uint8_t val) {}
void ahx_absolute_y(State *state, uint8_t val) {}
void ahx_indirect_y(State *state, uint8_t val) {}
