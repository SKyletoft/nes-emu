#include <stdint.h>

typedef union {
	struct {
		uint8_t C       : 1;
		uint8_t Z       : 1;
		uint8_t I       : 1;
		uint8_t D       : 1;
		uint8_t B       : 1;
		uint8_t _unused : 1;
		uint8_t V       : 1;
		uint8_t N       : 1;
	};
	uint8_t raw;
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

#define ACCUMULATOR(fn)                                                                          \
	void fn##_accumulator(State *state) {                                                    \
		fn##_impl(state, &state->cpu.a);                                                 \
	}

#define IMMEDIATE(fn)                                                                            \
	void fn##_immediate(State *state, uint8_t val) {                                         \
		fn##_impl(state, val);                                                           \
	}

#define ZERO_PAGE(fn)                                                                            \
	void fn##_zero_page(State *state, uint8_t offset) {                                      \
		uint8_t val = state_get_mem(state, (uint16_t) offset);                           \
		fn##_impl(state, val);                                                           \
	}

#define ZERO_PAGE_RMW(fn)                                                                        \
	void fn##_zero_page(State *state, uint8_t offset) {                                      \
		uint8_t val = state_get_mem(state, (uint16_t) offset);                           \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, (uint16_t) offset, val);                                    \
	}

#define ZERO_PAGE_X(fn)                                                                          \
	void fn##_zero_page_x(State *state, uint8_t offset) {                                    \
		uint8_t val =                                                                    \
		    state_get_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF);  \
		fn##_impl(state, val);                                                           \
	}
#define ZERO_PAGE_X_RMW(fn)                                                                      \
	void fn##_zero_page_x(State *state, uint8_t offset) {                                    \
		uint8_t val =                                                                    \
		    state_get_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF);  \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, (uint16_t) offset, val);                                    \
	}

#define ZERO_PAGE_Y(fn)                                                                          \
	void fn##_zero_page_y(State *state, uint8_t offset) {                                    \
		uint8_t val =                                                                    \
		    state_get_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF);  \
		fn##_impl(state, val);                                                           \
	}

#define ABSOLUTE(fn)                                                                             \
	void fn##_absolute(State *state, uint16_t adr) {                                         \
		uint8_t val = state_get_mem(state, adr);                                         \
		fn##_impl(state, val);                                                           \
	}

#define ABSOLUTE_RMW(fn)                                                                         \
	void fn##_absolute(State *state, uint16_t adr) {                                         \
		uint8_t val = state_get_mem(state, adr);                                         \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, adr, val);                                                  \
	}

#define ABSOLUTE_X(fn)                                                                           \
	void fn##_absolute_x(State *state, uint16_t adr) {                                       \
		uint8_t val = state_get_mem(state, (uint16_t) state->cpu.x + adr);               \
		fn##_impl(state, val);                                                           \
	}

#define ABSOLUTE_X_RMW(fn)                                                                       \
	void fn##_absolute_x(State *state, uint16_t adr) {                                       \
		uint8_t val = state_get_mem(state, (uint16_t) state->cpu.x + adr);               \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, (uint16_t) state->cpu.x + adr, val);                        \
	}

#define ABSOLUTE_Y(fn)                                                                           \
	void fn##_absolute_y(State *state, uint16_t adr) {                                       \
		uint8_t val = state_get_mem(state, (uint16_t) state->cpu.y + adr);               \
		fn##_impl(state, val);                                                           \
	}

#define ABSOLUTE_Y_RMW(fn)                                                                       \
	void fn##_absolute_y(State *state, uint16_t adr) {                                       \
		uint8_t val = state_get_mem(state, (uint16_t) state->cpu.y + adr);               \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, (uint16_t) state->cpu.y + adr, val);                        \
	}

#define INDIRECT_X(fn)                                                                           \
	void fn##_indirect_x(State *state, uint8_t adr) {                                        \
		uint8_t tmp = state_get_mem(state, (uint16_t) (state->cpu.x + adr) & 0xFF);      \
		uint16_t adr2 =                                                                  \
		    (uint16_t) (state_get_mem(state, (uint16_t) tmp)                             \
		                | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);       \
		uint8_t val = state_get_mem(state, adr2);                                        \
		fn##_impl(state, val);                                                           \
	}

#define INDIRECT_Y(fn)                                                                           \
	void fn##_indirect_y(State *state, uint8_t adr) {                                        \
		uint8_t tmp = state_get_mem(state, (uint16_t) (state->cpu.y + adr) & 0xFF);      \
		uint16_t adr2 =                                                                  \
		    (uint16_t) (state_get_mem(state, (uint16_t) tmp)                             \
		                | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);       \
		uint8_t val = state_get_mem(state, adr2);                                        \
		fn##_impl(state, val);                                                           \
	}
