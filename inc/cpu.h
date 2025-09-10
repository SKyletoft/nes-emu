#pragma once
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
