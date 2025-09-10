#include "inc/cpu.h"

#include <stdint.h>

int test() {
	return 5;
}

void adc_immediate(Cpu *cpu, uint8_t val) {
	uint16_t res = (uint16_t) cpu->a + (uint16_t) cpu->p.C + (uint16_t) val;

	cpu->p.C = res > 255;
	cpu->p.Z = 0 == (uint8_t) res;
	cpu->p.V = ((res ^ (uint16_t) cpu->a) & (res ^ (uint16_t) val) & (uint16_t) 0x80) != 0;
	cpu->p.N = (res & (1 >> 7)) >> 7;
	cpu->a   = (uint8_t) res;
}
