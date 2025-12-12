#include <stdint.h>

#ifndef STATIC_INLINE
#define STATIC_INLINE
#endif

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
	uint8_t ctrl;
	uint8_t mask;
	uint8_t status;
	uint8_t oam_adr;
	uint8_t oam_data;
	struct {
		uint8_t x;
		uint8_t y;
	} scroll;
	struct {
		uint8_t high;
		uint8_t low;
	} adr;
	uint8_t data;

	uint16_t scanline;
	uint16_t dot;
	uint8_t vram[2048];
	uint8_t oam[256];
} Ppu;

typedef uint8_t Bitmap[240][256];

typedef struct {
	Cpu cpu;
	Ppu ppu;
	/* Mapper */ void *rom;
	uint8_t ram[2048];
	uint8_t bus;
	/* Arc<Mutex<Bitmap>> */ void *output_texture;
	Bitmap current_texture;
} State;

uint8_t state_get_mem(State *state, uint16_t adr);
void state_set_mem(State *state, uint16_t adr, uint8_t val);
void state_check_interrupt(State *state);
void state_step_ppu(State *state);
void state_step_ppu_many(State *state, uint32_t times);

#define ACCUMULATOR(fn)                                                                          \
	STATIC_INLINE void fn##_accumulator(State *state) {                                      \
		fn##_impl(state, &state->cpu.a);                                                 \
		state->cpu.pc += 1;                                                              \
		state_step_ppu_many(state, 2);                                                   \
	}

#define IMMEDIATE(fn)                                                                            \
	STATIC_INLINE void fn##_immediate(State *state, uint8_t val) {                           \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 2);                                                   \
	}

#define ZERO_PAGE(fn)                                                                            \
	STATIC_INLINE void fn##_zero_page(State *state, uint8_t offset) {                        \
		uint8_t val = state_get_mem(state, (uint16_t) offset);                           \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 3);                                                   \
	}

#define ZERO_PAGE_RMW(fn)                                                                        \
	STATIC_INLINE void fn##_zero_page(State *state, uint8_t offset) {                        \
		uint8_t val = state_get_mem(state, (uint16_t) offset);                           \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, (uint16_t) offset, val);                                    \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 5);                                                   \
	}

#define ZERO_PAGE_X(fn)                                                                          \
	STATIC_INLINE void fn##_zero_page_x(State *state, uint8_t offset) {                      \
		uint8_t val =                                                                    \
		    state_get_mem(state, ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF);  \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 4);                                                   \
	}

#define ZERO_PAGE_X_RMW(fn)                                                                      \
	STATIC_INLINE void fn##_zero_page_x(State *state, uint8_t offset) {                      \
		uint16_t actual_adr = ((uint16_t) state->cpu.x + (uint16_t) offset) & 0xFF;      \
		uint8_t val         = state_get_mem(state, actual_adr);                          \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, actual_adr, val);                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 6);                                                   \
	}

#define ZERO_PAGE_Y(fn)                                                                          \
	STATIC_INLINE void fn##_zero_page_y(State *state, uint8_t offset) {                      \
		uint8_t val =                                                                    \
		    state_get_mem(state, ((uint16_t) state->cpu.y + (uint16_t) offset) & 0xFF);  \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 4);                                                   \
	}

#define ABSOLUTE(fn)                                                                             \
	STATIC_INLINE void fn##_absolute(State *state, uint16_t adr) {                           \
		uint8_t val = state_get_mem(state, adr);                                         \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 4);                                                   \
	}

#define ABSOLUTE_RMW(fn)                                                                         \
	STATIC_INLINE void fn##_absolute(State *state, uint16_t adr) {                           \
		uint8_t val = state_get_mem(state, adr);                                         \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, adr, val);                                                  \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 6);                                                   \
	}

#define ABSOLUTE_X(fn)                                                                           \
	STATIC_INLINE void fn##_absolute_x(State *state, uint16_t adr) {                         \
		uint16_t actual_adr = (uint16_t) state->cpu.x + adr;                             \
		bool page_crossed   = state->cpu.x + (adr & 0xFF) > 0xFF;                        \
		uint8_t val         = state_get_mem(state, actual_adr);                          \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 4 + page_crossed);                                    \
	}

#define ABSOLUTE_X_RMW(fn)                                                                       \
	STATIC_INLINE void fn##_absolute_x(State *state, uint16_t adr) {                         \
		uint16_t actual_adr = state->cpu.x + adr;                                        \
		uint8_t val         = state_get_mem(state, actual_adr);                          \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, actual_adr, val);                                           \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 7);                                                   \
	}

#define ABSOLUTE_Y(fn)                                                                           \
	STATIC_INLINE void fn##_absolute_y(State *state, uint16_t adr) {                         \
		uint16_t actual_adr = (uint16_t) state->cpu.y + adr;                             \
		bool page_crossed   = state->cpu.y + (adr & 0xFF) > 0xFF;                        \
		uint8_t val         = state_get_mem(state, actual_adr);                          \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 4 + page_crossed);                                    \
	}

#define ABSOLUTE_Y_RMW(fn)                                                                       \
	STATIC_INLINE void fn##_absolute_y(State *state, uint16_t adr) {                         \
		uint16_t actual_adr = state->cpu.y + adr;                                        \
		uint8_t val         = state_get_mem(state, actual_adr);                          \
		fn##_impl(state, &val);                                                          \
		state_set_mem(state, actual_adr, val);                                           \
		state->cpu.pc += 3;                                                              \
		state_step_ppu_many(state, 7);                                                   \
	}

#define INDIRECT_X(fn)                                                                           \
	STATIC_INLINE void fn##_indirect_x(State *state, uint8_t adr) {                          \
		uint8_t tmp = state_get_mem(state, (uint16_t) (state->cpu.x + adr) & 0xFF);      \
		uint16_t adr2 =                                                                  \
		    (uint16_t) (state_get_mem(state, (uint16_t) tmp)                             \
				| state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);       \
		uint8_t val = state_get_mem(state, adr2);                                        \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 6);                                                   \
	}

#define INDIRECT_Y(fn)                                                                           \
	STATIC_INLINE void fn##_indirect_y(State *state, uint8_t adr) {                          \
		uint8_t tmp = state_get_mem(state, (uint16_t) (state->cpu.y + adr) & 0xFF);      \
		uint16_t adr2 =                                                                  \
		    (uint16_t) (state_get_mem(state, (uint16_t) tmp)                             \
				| state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);       \
		bool taken  = (adr2 & 0xFF) == 0;                                                \
		uint8_t val = state_get_mem(state, adr2);                                        \
		fn##_impl(state, val);                                                           \
		state->cpu.pc += 2;                                                              \
		state_step_ppu_many(state, 5 + taken);                                           \
	}
