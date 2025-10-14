#include <stdalign.h>
#include <stdint.h>

// Ensure consistent struct layout
#pragma pack(push, 1)

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
	uint8_t frame;
	uint8_t cycles;
	uint8_t data_cache;
	uint8_t _pad[3];
} Ppu;

typedef uint8_t Bitmap[240][256];

typedef enum {
	InterruptTiming_Clear,
	InterruptTiming_Waiting,
	InterruptTiming_Ready
} InterruptTiming;

typedef struct {
	Cpu cpu;
	Ppu ppu;
	/* Apu */ void *apu;
	/* Controller */ void *controller1;
	/* Controller */ void *controller2;
	/* Mapper */ void *rom;
	uint8_t ram[2048];
	uint8_t cpu_bus;
	uint8_t ppu_bus;
	/* Arc<Mutex<Bitmap>> */ void *output_texture;
	Bitmap current_texture;
	uint64_t cycles;
	InterruptTiming interrupt_requested;
} State;

#pragma pack(pop)

uint8_t state_get_mem(State *state, uint16_t adr);
void state_set_mem(State *state, uint16_t adr, uint8_t val);
void state_check_interrupt(State *state);
void state_step_ppu(State *state);
void state_step_ppu_many(State *state, uint32_t times);

/* ------------------------------------------------------------*
 * Macro definitions for instruction handlers
 * ------------------------------------------------------------------ */

/* These macros are used in the C source files to generate
 * instruction handler functions.  The original project
 * defined them in a separate header that was omitted.
 * For the purposes of compiling the current code base,
 * we provide minimal definitions that expand to empty
 * bodies.  The actual implementation of the handlers
 * is provided elsewhere in the C source files.
 */

#define IMMEDIATE(name)       /* empty */
#define ZERO_PAGE(name)       /* empty */
#define ZERO_PAGE_X(name)     /* empty */
#define ABSOLUTE(name)        /* empty */
#define ABSOLUTE_X(name)      /* empty */
#define ABSOLUTE_Y(name)      /* empty */
#define INDIRECT_X(name)      /* empty */
#define INDIRECT_Y(name)      /* empty */
#define ACCUMULATOR(name)     /* empty */
#define ZERO_PAGE_RMW(name)   /* empty */
#define ZERO_PAGE_X_RMW(name) /* empty */
#define ABSOLUTE_RMW(name)    /* empty */
#define ABSOLUTE_X_RMW(name)  /* empty */
#define ABSOLUTE_Y_RMW(name)  /* empty */
