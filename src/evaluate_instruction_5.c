#include "interface.h"

// C-implementations of NES instructions

void lax_zero_page(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void lax_zero_page_y(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void lax_absolute(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void lax_absolute_y(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void lax_indirect_x(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void lax_indirect_y(State *state, uint8_t val) {
	state->cpu.a = val;
	state->cpu.x = val;
	// Update flags
	state->cpu.p.Z = (val == 0);
	state->cpu.p.N = ((val & 0x80) != 0);
	state->cpu.pc += 2;
}

void sax_zero_page(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

void sax_zero_page_y(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

void sax_absolute(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

void sax_indirect_x(State *state, uint8_t val) {
	uint8_t result = state->cpu.a & state->cpu.x;
	state_set_mem(state, val, result);
	state->cpu.pc += 2;
}

void dcp_zero_page(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_zero_page_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_absolute(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_absolute_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_absolute_y(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_indirect_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void dcp_indirect_y(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state->cpu.a - result;
	state->cpu.p.C = (temp < state->cpu.a);
	state->cpu.p.Z = (temp == 0);
	state->cpu.p.N = ((temp & 0x80) != 0);
	state->cpu.pc += 2;
}

void isc_zero_page(State *state, uint8_t val) {
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

void isc_zero_page_x(State *state, uint8_t val) {
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

void isc_absolute(State *state, uint8_t val) {
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

void isc_absolute_x(State *state, uint8_t val) {
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

void isc_absolute_y(State *state, uint8_t val) {
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

void isc_indirect_x(State *state, uint8_t val) {
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

void isc_indirect_y(State *state, uint8_t val) {
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

void rla_zero_page(State *state, uint8_t val) {
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

void rla_zero_page_x(State *state, uint8_t val) {
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

void rla_absolute(State *state, uint8_t val) {
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

void rla_absolute_x(State *state, uint8_t val) {
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

void rla_absolute_y(State *state, uint8_t val) {
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

void rla_indirect_x(State *state, uint8_t val) {
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

void rla_indirect_y(State *state, uint8_t val) {
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

void rra_zero_page(State *state, uint8_t val) {
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

void rra_zero_page_x(State *state, uint8_t val) {
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

void rra_absolute(State *state, uint8_t val) {
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

void rra_absolute_x(State *state, uint8_t val) {
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

void rra_absolute_y(State *state, uint8_t val) {
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

void rra_indirect_x(State *state, uint8_t val) {
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

void rra_indirect_y(State *state, uint8_t val) {
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

void slo_zero_page(State *state, uint8_t val) {
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

void slo_zero_page_x(State *state, uint8_t val) {
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

void slo_absolute(State *state, uint8_t val) {
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

void slo_absolute_x(State *state, uint8_t val) {
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

void slo_absolute_y(State *state, uint8_t val) {
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

void slo_indirect_x(State *state, uint8_t val) {
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

void slo_indirect_y(State *state, uint8_t val) {
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

void sre_zero_page(State *state, uint8_t val) {
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

void sre_zero_page_x(State *state, uint8_t val) {
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

void sre_absolute(State *state, uint8_t val) {
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

void sre_absolute_x(State *state, uint8_t val) {
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

void sre_absolute_y(State *state, uint8_t val) {
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

void sre_indirect_x(State *state, uint8_t val) {
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

void sre_indirect_y(State *state, uint8_t val) {
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

void anc(State *state, uint8_t val) {
	// AND with accumulator
	state->cpu.a &= val;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);

	// Set carry flag to bit 7 of accumulator
	state->cpu.p.C = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

void alr(State *state, uint8_t val) {
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

void arr(State *state, uint8_t val) {
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

void axs(State *state, uint8_t val) {
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

void las(State *state, uint8_t val) {
	// AND with accumulator and store in A, X, and S
	state->cpu.a &= val;
	state->cpu.x = state->cpu.a;
	state->cpu.s = state->cpu.a;

	// Update flags
	state->cpu.p.Z = (state->cpu.a == 0);
	state->cpu.p.N = ((state->cpu.a & 0x80) != 0);
	state->cpu.pc += 2;
}

void tas(State *state, uint8_t val) {
	// AND accumulator with X register and store in S
	uint8_t temp = state->cpu.a & state->cpu.x;
	state->cpu.s = temp;

	// Store S in memory
	state_set_mem(state, val, temp);
	state->cpu.pc += 2;
}

void shy(State *state, uint8_t val) {
	// Store Y register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store Y register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.y);
	state->cpu.pc += 2;
}

void shx(State *state, uint8_t val) {
	// Store X register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store X register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.x);
	state->cpu.pc += 2;
}

void ahx_absolute_y(State *state, uint8_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.a & state->cpu.x);
	state->cpu.pc += 2;
}

void ahx_indirect_y(State *state, uint8_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state->cpu.a & state->cpu.x);
	state->cpu.pc += 2;
}
