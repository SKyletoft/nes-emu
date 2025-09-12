#include "inc/cpu.h"

#include <stdint.h>

// C-implementations of NES instructions

int test() {
	return 5;
}

void adc_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_zero_page(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_zero_page_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_absolute(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_absolute_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_absolute_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_indirect_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void adc_indirect_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void and_immediate(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_zero_page(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_absolute(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_absolute_y(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_indirect_x(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void and_indirect_y(Cpu *cpu, uint8_t val) {
	cpu->a &= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void asl_accumulator(Cpu *cpu) {
	cpu->p.C = (cpu->a & 0x80) >> 7;
	cpu->a <<= 1;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void asl_zero_page(Cpu *cpu, uint8_t val) {
	cpu->p.C = (val & 0x80) >> 7;
	val <<= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void asl_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->p.C = (val & 0x80) >> 7;
	val <<= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void asl_absolute(Cpu *cpu, uint8_t val) {
	cpu->p.C = (val & 0x80) >> 7;
	val <<= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void asl_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->p.C = (val & 0x80) >> 7;
	val <<= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void bcc(Cpu *cpu, int8_t offset) {
	if (!cpu->p.C) {
		cpu->pc += offset;
	}
}

void bcs(Cpu *cpu, int8_t offset) {
	if (cpu->p.C) {
		cpu->pc += offset;
	}
}

void beq(Cpu *cpu, int8_t offset) {
	if (cpu->p.Z) {
		cpu->pc += offset;
	}
}

void bit_zero_page(Cpu *cpu, uint8_t val) {
	cpu->p.Z = 0 == (cpu->a & val);
	cpu->p.V = (val & 0x40) >> 6;
	cpu->p.N = (val & 0x80) >> 7;
}

void bit_absolute(Cpu *cpu, uint8_t val) {
	cpu->p.Z = 0 == (cpu->a & val);
	cpu->p.V = (val & 0x40) >> 6;
	cpu->p.N = (val & 0x80) >> 7;
}

void bmi(Cpu *cpu, int8_t offset) {
	if (cpu->p.N) {
		cpu->pc += offset;
	}
}

void bne(Cpu *cpu, int8_t offset) {
	if (!cpu->p.Z) {
		cpu->pc += offset;
	}
}

void bpl(Cpu *cpu, int8_t offset) {
	if (!cpu->p.N) {
		cpu->pc += offset;
	}
}

void brk(Cpu *cpu) {
	// BRK is a complex instruction that pushes PC+2 and status flags
	// This is a simplified version for demonstration
	cpu->pc++;
}

void bvc(Cpu *cpu, int8_t offset) {
	if (!cpu->p.V) {
		cpu->pc += offset;
	}
}

void bvs(Cpu *cpu, int8_t offset) {
	if (cpu->p.V) {
		cpu->pc += offset;
	}
}

void clc(Cpu *cpu) {
	cpu->p.C = 0;
}

void cld(Cpu *cpu) {
	cpu->p.D = 0;
}

void cli(Cpu *cpu) {
	cpu->p.I = 0;
}

void clv(Cpu *cpu) {
	cpu->p.V = 0;
}

void cmp_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_zero_page(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_zero_page_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_absolute(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_absolute_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_absolute_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_indirect_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cmp_indirect_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpx_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->x - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpx_zero_page(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->x - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpx_absolute(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->x - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpy_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->y - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpy_zero_page(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->y - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void cpy_absolute(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->y - (uint16_t) val;
	cpu->p.C     = res < 256;
	cpu->p.Z     = 0 == (uint8_t) res;
	cpu->p.N     = (res & 0x80) >> 7;
}

void dec_zero_page(Cpu *cpu, uint8_t val) {
	val--;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void dec_zero_page_x(Cpu *cpu, uint8_t val) {
	val--;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void dec_absolute(Cpu *cpu, uint8_t val) {
	val--;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void dec_absolute_x(Cpu *cpu, uint8_t val) {
	val--;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void dex(Cpu *cpu) {
	cpu->x--;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void dey(Cpu *cpu) {
	cpu->y--;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void eor_immediate(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_zero_page(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_absolute(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_absolute_y(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_indirect_x(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void eor_indirect_y(Cpu *cpu, uint8_t val) {
	cpu->a ^= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void inc_zero_page(Cpu *cpu, uint8_t val) {
	val++;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void inc_zero_page_x(Cpu *cpu, uint8_t val) {
	val++;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void inc_absolute(Cpu *cpu, uint8_t val) {
	val++;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void inc_absolute_x(Cpu *cpu, uint8_t val) {
	val++;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void inx(Cpu *cpu) {
	cpu->x++;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void iny(Cpu *cpu) {
	cpu->y++;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void jmp_absolute(Cpu *cpu, uint16_t addr) {
	cpu->pc = addr;
}

void jmp_indirect(Cpu *cpu, uint16_t addr) {
	cpu->pc = addr;
}

void jsr(Cpu *cpu, uint16_t addr) {
	// Push return address (pc + 2)
	cpu->pc = addr;
}

void lda_immediate(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_zero_page(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_absolute(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_absolute_y(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_indirect_x(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lda_indirect_y(Cpu *cpu, uint8_t val) {
	cpu->a   = val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ldx_immediate(Cpu *cpu, uint8_t val) {
	cpu->x   = val;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void ldx_zero_page(Cpu *cpu, uint8_t val) {
	cpu->x   = val;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void ldx_zero_page_y(Cpu *cpu, uint8_t val) {
	cpu->x   = val;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void ldx_absolute(Cpu *cpu, uint8_t val) {
	cpu->x   = val;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void ldx_absolute_y(Cpu *cpu, uint8_t val) {
	cpu->x   = val;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void ldy_immediate(Cpu *cpu, uint8_t val) {
	cpu->y   = val;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void ldy_zero_page(Cpu *cpu, uint8_t val) {
	cpu->y   = val;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void ldy_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->y   = val;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void ldy_absolute(Cpu *cpu, uint8_t val) {
	cpu->y   = val;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void ldy_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->y   = val;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void lsr_accumulator(Cpu *cpu) {
	cpu->p.C = cpu->a & 0x01;
	cpu->a >>= 1;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lsr_zero_page(Cpu *cpu, uint8_t val) {
	cpu->p.C = val & 0x01;
	val >>= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void lsr_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->p.C = val & 0x01;
	val >>= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void lsr_absolute(Cpu *cpu, uint8_t val) {
	cpu->p.C = val & 0x01;
	val >>= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void lsr_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->p.C = val & 0x01;
	val >>= 1;
	cpu->p.Z = 0 == val;
	cpu->p.N = (val & 0x80) >> 7;
}

void nop(Cpu *cpu) {
	// No operation
}

void ora_immediate(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_zero_page(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_zero_page_x(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_absolute(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_absolute_x(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_absolute_y(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_indirect_x(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void ora_indirect_y(Cpu *cpu, uint8_t val) {
	cpu->a |= val;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void pha(Cpu *cpu) {
	// Push accumulator onto stack
}

void php(Cpu *cpu) {
	// Push processor status onto stack
}

void pla(Cpu *cpu) {
	// Pull accumulator from stack
}

void plp(Cpu *cpu) {
	// Pull processor status from stack
}

void rol_accumulator(Cpu *cpu) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = (cpu->a & 0x80) >> 7;
	cpu->a        = ((cpu->a << 1) | carry) != 0;
	cpu->p.Z      = 0 == cpu->a;
	cpu->p.N      = (cpu->a & 0x80) >> 7;
}

void rol_zero_page(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void rol_zero_page_x(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void rol_absolute(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void rol_absolute_x(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = (val & 0x80) >> 7;
	val           = ((val << 1) | carry) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void ror_accumulator(Cpu *cpu) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = cpu->a & 0x01;
	cpu->a        = ((carry << 7) | (cpu->a >> 1)) != 0;
	cpu->p.Z      = 0 == cpu->a;
	cpu->p.N      = (cpu->a & 0x80) >> 7;
}

void ror_zero_page(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void ror_zero_page_x(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void ror_absolute(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void ror_absolute_x(Cpu *cpu, uint8_t val) {
	uint8_t carry = cpu->p.C;
	cpu->p.C      = val & 0x01;
	val           = ((carry << 7) | (val >> 1)) != 0;
	cpu->p.Z      = 0 == val;
	cpu->p.N      = (val & 0x80) >> 7;
}

void rti(Cpu *cpu) {
	// Return from interrupt
}

void rts(Cpu *cpu) {
	// Return from subroutine
}

void sbc_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_zero_page(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_zero_page_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_absolute(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_absolute_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_absolute_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_indirect_x(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sbc_indirect_y(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a - (uint16_t) val - (uint16_t) (1 - cpu->p.C);

	cpu->p.C = res < 256;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & 0x80) >> 7;
	cpu->a   = (uint8_t) res;
}

void sec(Cpu *cpu) {
	cpu->p.C = 1;
}

void sed(Cpu *cpu) {
	cpu->p.D = 1;
}

void sei(Cpu *cpu) {
	cpu->p.I = 1;
}

void sta_zero_page(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_zero_page_x(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute_x(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_absolute_y(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_indirect_x(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void sta_indirect_y(Cpu *cpu, uint8_t val) {
	// Store accumulator in memory
}

void stx_zero_page(Cpu *cpu, uint8_t val) {
	// Store X register in memory
}

void stx_zero_page_y(Cpu *cpu, uint8_t val) {
	// Store X register in memory
}

void stx_absolute(Cpu *cpu, uint8_t val) {
	// Store X register in memory
}

void sty_zero_page(Cpu *cpu, uint8_t val) {
	// Store Y register in memory
}

void sty_zero_page_x(Cpu *cpu, uint8_t val) {
	// Store Y register in memory
}

void sty_absolute(Cpu *cpu, uint8_t val) {
	// Store Y register in memory
}

void tax(Cpu *cpu) {
	cpu->x   = cpu->a;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void tay(Cpu *cpu) {
	cpu->y   = cpu->a;
	cpu->p.Z = 0 == cpu->y;
	cpu->p.N = (cpu->y & 0x80) >> 7;
}

void tsx(Cpu *cpu) {
	cpu->x   = cpu->s;
	cpu->p.Z = 0 == cpu->x;
	cpu->p.N = (cpu->x & 0x80) >> 7;
}

void txa(Cpu *cpu) {
	cpu->a   = cpu->x;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void txs(Cpu *cpu) {
	cpu->s = cpu->x;
}

void tya(Cpu *cpu) {
	cpu->a   = cpu->y;
	cpu->p.Z = 0 == cpu->a;
	cpu->p.N = (cpu->a & 0x80) >> 7;
}

void lax_zero_page(Cpu *cpu, uint8_t val) {}
void lax_zero_page_y(Cpu *cpu, uint8_t val) {}
void lax_absolute(Cpu *cpu, uint8_t val) {}
void lax_absolute_y(Cpu *cpu, uint8_t val) {}
void lax_indirect_x(Cpu *cpu, uint8_t val) {}
void lax_indirect_y(Cpu *cpu, uint8_t val) {}
void sax_zero_page(Cpu *cpu, uint8_t val) {}
void sax_zero_page_y(Cpu *cpu, uint8_t val) {}
void sax_absolute(Cpu *cpu, uint8_t val) {}
void sax_indirect_x(Cpu *cpu, uint8_t val) {}
void dcp_zero_page(Cpu *cpu, uint8_t val) {}
void dcp_zero_page_x(Cpu *cpu, uint8_t val) {}
void dcp_absolute(Cpu *cpu, uint8_t val) {}
void dcp_absolute_x(Cpu *cpu, uint8_t val) {}
void dcp_absolute_y(Cpu *cpu, uint8_t val) {}
void dcp_indirect_x(Cpu *cpu, uint8_t val) {}
void dcp_indirect_y(Cpu *cpu, uint8_t val) {}
void isc_zero_page(Cpu *cpu, uint8_t val) {}
void isc_zero_page_x(Cpu *cpu, uint8_t val) {}
void isc_absolute(Cpu *cpu, uint8_t val) {}
void isc_absolute_x(Cpu *cpu, uint8_t val) {}
void isc_absolute_y(Cpu *cpu, uint8_t val) {}
void isc_indirect_x(Cpu *cpu, uint8_t val) {}
void isc_indirect_y(Cpu *cpu, uint8_t val) {}
void rla_zero_page(Cpu *cpu, uint8_t val) {}
void rla_zero_page_x(Cpu *cpu, uint8_t val) {}
void rla_absolute(Cpu *cpu, uint8_t val) {}
void rla_absolute_x(Cpu *cpu, uint8_t val) {}
void rla_absolute_y(Cpu *cpu, uint8_t val) {}
void rla_indirect_x(Cpu *cpu, uint8_t val) {}
void rla_indirect_y(Cpu *cpu, uint8_t val) {}
void rra_zero_page(Cpu *cpu, uint8_t val) {}
void rra_zero_page_x(Cpu *cpu, uint8_t val) {}
void rra_absolute(Cpu *cpu, uint8_t val) {}
void rra_absolute_x(Cpu *cpu, uint8_t val) {}
void rra_absolute_y(Cpu *cpu, uint8_t val) {}
void rra_indirect_x(Cpu *cpu, uint8_t val) {}
void rra_indirect_y(Cpu *cpu, uint8_t val) {}
void slo_zero_page(Cpu *cpu, uint8_t val) {}
void slo_zero_page_x(Cpu *cpu, uint8_t val) {}
void slo_absolute(Cpu *cpu, uint8_t val) {}
void slo_absolute_x(Cpu *cpu, uint8_t val) {}
void slo_absolute_y(Cpu *cpu, uint8_t val) {}
void slo_indirect_x(Cpu *cpu, uint8_t val) {}
void slo_indirect_y(Cpu *cpu, uint8_t val) {}
void sre_zero_page(Cpu *cpu, uint8_t val) {}
void sre_zero_page_x(Cpu *cpu, uint8_t val) {}
void sre_absolute(Cpu *cpu, uint8_t val) {}
void sre_absolute_x(Cpu *cpu, uint8_t val) {}
void sre_absolute_y(Cpu *cpu, uint8_t val) {}
void sre_indirect_x(Cpu *cpu, uint8_t val) {}
void sre_indirect_y(Cpu *cpu, uint8_t val) {}
void anc(Cpu *cpu, uint8_t val) {}
void alr(Cpu *cpu, uint8_t val) {}
void arr(Cpu *cpu, uint8_t val) {}
void axs(Cpu *cpu, uint8_t val) {}
void las(Cpu *cpu, uint8_t val) {}
void tas(Cpu *cpu, uint8_t val) {}
void shy(Cpu *cpu, uint8_t val) {}
void shx(Cpu *cpu, uint8_t val) {}
void ahx_absolute_y(Cpu *cpu, uint8_t val) {}
void ahx_indirect_y(Cpu *cpu, uint8_t val) {}
