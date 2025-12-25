use emu_core::interpret::State;
use paste::paste;

macro_rules! accumulator {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _accumulator>](state: &mut State) {
				[<$fn _impl>](state, &mut state.cpu.a);
				state.cpu.pc += 1;
				state_step_ppu_many(state, 2);
			}
		}
	};
}

macro_rules! immediate {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _immediate>](state: &mut State, val: u8) {
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 2);
			}
		}
	};
}

macro_rules! zero_page {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page>](state: &mut State, offset: u8) {
				let val = state_get_mem(state, offset as u16);
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 3);
			}
		}
	};
}

macro_rules! zero_page_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page>](state: &mut State, offset: u8) {
				let mut val = state_get_mem(state, offset as u16);
				[<$fn _impl>](state, &mut val);
				state_set_mem(state, offset as u16, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 5);
			}
		}
	};
}

macro_rules! zero_page_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page_x>](state: &mut State, offset: u8) {
				let adr = state.cpu.x.wrapping_add(offset) as u16;
				let val = state_get_mem(state, adr & 0x00FF);
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 4);
			}
		}
	};
}

macro_rules! zero_page_x_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page_x>](state: &mut State, offset: u8) {
				let actual_adr = (state.cpu.x.wrapping_add(offset)) as u16 & 0x00FF;
				let mut val = state_get_mem(state, actual_adr);
				[<$fn _impl>](state, &mut val);
				state_set_mem(state, actual_adr, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 6);
			}
		}
	};
}

macro_rules! zero_page_y {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page_y>](state: &mut State, offset: u8) {
				let adr = state.cpu.y.wrapping_add(offset) as u16;
				let val = state_get_mem(state, adr & 0x00FF);
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 4);
			}
		}
	};
}

macro_rules! absolute {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute>](state: &mut State, adr: u16) {
				let val = state_get_mem(state, adr);
				[<$fn _impl>](state, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 4);
			}
		}
	};
}

macro_rules! absolute_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute>](state: &mut State, adr: u16) {
				let mut val = state_get_mem(state, adr);
				[<$fn _impl>](state, &mut val);
				state_set_mem(state, adr, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 6);
			}
		}
	};
}

macro_rules! absolute_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_x>](state: &mut State, adr: u16) {
				let actual_adr = adr.wrapping_add(state.cpu.x as u16);
				let page_crossed = (state.cpu.x as u16 + (adr & 0x00FF)) > 0x00FF;
				let val = state_get_mem(state, actual_adr);
				[<$fn _impl>](state, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 4 + page_crossed as u32);
			}
		}
	};
}

macro_rules! absolute_x_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_x>](state: &mut State, adr: u16) {
				let actual_adr = adr.wrapping_add(state.cpu.x as u16);
				let mut val = state_get_mem(state, actual_adr);
				[<$fn _impl>](state, &mut val);
				state_set_mem(state, actual_adr, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 7);
			}
		}
	};
}

macro_rules! absolute_y {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_y>](state: &mut State, adr: u16) {
				let actual_adr = adr.wrapping_add(state.cpu.y as u16);
				let page_crossed = (state.cpu.y as u16 + (adr & 0x00FF)) > 0x00FF;
				let val = state_get_mem(state, actual_adr);
				[<$fn _impl>](state, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 4 + page_crossed as u32);
			}
		}
	};
}

macro_rules! absolute_y_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_y>](state: &mut State, adr: u16) {
				let actual_adr = adr.wrapping_add(state.cpu.y as u16);
				let mut val = state_get_mem(state, actual_adr);
				[<$fn _impl>](state, &mut val);
				state_set_mem(state, actual_adr, val);
				state.cpu.pc += 3;
				state_step_ppu_many(state, 7);
			}
		}
	};
}

macro_rules! indirect_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _indirect_x>](state: &mut State, adr: u8) {
				let tmp = state_get_mem(state, state.cpu.x.wrapping_add(adr) as u16 & 0x00FF);
				let lo = state_get_mem(state, tmp as u16);
				let hi = state_get_mem(state, (tmp.wrapping_add(1)) as u16 & 0x00FF);
				let adr2 = (lo as u16) | ((hi as u16) << 8);
				let val = state_get_mem(state, adr2);
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 6);
			}
		}
	};
}

macro_rules! indirect_y {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _indirect_y>](state: &mut State, adr: u8) {
				let tmp = state_get_mem(state, state.cpu.y.wrapping_add(adr) as u16 & 0x00FF);
				let lo = state_get_mem(state, tmp as u16);
				let hi = state_get_mem(state, (tmp.wrapping_add(1)) as u16 & 0x00FF);
				let adr2 = (lo as u16) | ((hi as u16) << 8);
				let taken = (adr2 & 0x00FF) == 0;
				let val = state_get_mem(state, adr2);
				[<$fn _impl>](state, val);
				state.cpu.pc += 2;
				state_step_ppu_many(state, 5 + taken as u32);
			}
		}
	};
}

fn adc_impl(state: &mut State, val: u8) {
	let res = state.cpu.a as u16 + state.cpu.p.C as u16 + val as u16;
	state.cpu.p.set_c(res > 0xFF);
	state.cpu.p.set_z((res as u8) == 0);
	state.cpu.p.set_v(((res ^ state.cpu.a as u16) & (res ^ val as u16) & 0x80) != 0);
	state.cpu.p.set_n(((res & 0x80) >> 7) as u8);
	state.cpu.a = res as u8;
}

immediate!(adc);
zero_page!(adc);
zero_page_x!(adc);
absolute!(adc);
absolute_x!(adc);
absolute_y!(adc);
indirect_x!(adc);
indirect_y!(adc);

fn and_impl(state: &mut State, val: u8) {
	state.cpu.a &= val;
	state.cpu.p.set_z(state.cpu.a == 0);
	state.cpu.p.set_n(state.cpu.a & 0x80 != 0);
}

immediate!(and);
zero_page!(and);
zero_page_x!(and);
absolute!(and);
absolute_x!(and);
absolute_y!(and);
indirect_x!(and);
indirect_y!(and);

fn asl_impl(state: &mut State, val: &mut u8) {
	state.cpu.set_c(*val & 0x80 != 0);
	*val <<= 1;
	state.cpu.p.set_z(*val != 0);
	state.cpu.p.set_n(*val &0x80 != 0);
}

accumulator!(asl);
zero_page_rmw!(asl);
zero_page_x_rmw!(asl);
absolute_rmw!(asl);
absolute_x_rmw!(asl);

macro_rules! branch {
	(inst:$id, criterion:$expr) => fn $inst(state: &mut State, offset: i8) {
		let old_pc = state.cpu.pc;
		let taken = $criterion;
		let new_pc = old_pc + 2 + if taken {offset as u16} else {0};
		let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
		let cycles = 2 + taken as u8 + page_crossed as u8;
		state.cpu.pc = new_pc;
		state_step_ppu_many(state, cycles);
	}
}

branch!(bcs, state.cpu.p.c());
branch!(bcc, !state.cpu.p.c());
branch!(beq, state.cpu.p.z());
branch!(bne, !state.cpu.p.z());
branch!(bmi, state.cpu.p.n());
branch!(bpl, !state.cpu.p.n());
branch!(bvs, state.cpu.p.v());
branch!(bvc, !state.cpu.p.v());

fn bit_impl(state: &mut State, val: u8) {
	state.cpu.p.set_z(state.cpu.a & val == 0);
	state.cpu.p.set_v((val & 0x40) >> 6);
	state.cpu.p.set_n((val & 0x80) >> 7);
}

zero_page!(bit);
absolute!(bit);

fn brk(state: &mut State) {
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn clc(state: &mut State) {
	state.cpu.p.set_c(0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn cld(state: &mut State) {
	state.cpu.p.set_d(0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn cli(state: &mut State) {
	state.cpu.p.set_i(0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn clv(state: &mut State) {
	state.cpu.p.set_v(0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn cmp_impl(state: &mut State, val: u8) {
	let res   = state.cpu.a as u16 - val as u16;
	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state.cpu.p.set_n((res & 0x80) >> 7 != 0);
}

immediate!(cmp);
zero_page!(cmp);
zero_page_x!(cmp);
absolute!(cmp);
absolute_x!(cmp);
absolute_y!(cmp);
indirect_x!(cmp);
indirect_y!(cmp);

fn cpx_impl(state: &mut State, val: u8) {
	let res   = state.cpu.x as u16 - val as u16;
	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state.cpu.p.set_n((res & 0x80) >> 7);
}

immediate!(cpx);
zero_page!(cpx);
absolute!(cpx);

fn cpy_impl(state: &mut State, val: u8) {
	let res   = state.cpu.y as u16 - val as u16;
	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state.cpu.p.set_n(res & 0x80) >> 7;
}

immediate!(cpy);
zero_page!(cpy);
absolute!(cpy);

fn dec_impl(state: &mut State, val: &mut u8) {
	*val -= 1;
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) >> 7);
}

zero_page_rmw!(dec);
zero_page_x_rmw!(dec);
absolute_rmw!(dec);
absolute_x_rmw!(dec);

fn dex(state: &mut State) {
	state.cpu.x-= 1;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn dey(state: &mut State) {
	state.cpu.y-=1;
	state.cpu.p.set_z(0 == state.cpu.y);
	state.cpu.p.set_n((state.cpu.y & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn eor_impl(state: &mut State, val: u8) {
	state.cpu.a ^= val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
}

immediate!(eor);
zero_page!(eor);
zero_page_x!(eor);
absolute!(eor);
absolute_x!(eor);
absolute_y!(eor);
indirect_x!(eor);
indirect_y!(eor);

fn inc_impl(state: &mut State, val: &mut u8) {
	*val+= 1;
	state.cpu.p.set_z (0 == *val);
	state.cpu.p.set_n ((*val & 0x80) >> 7 != 0);
}

zero_page_rmw!(inc);
zero_page_x_rmw!(inc);
absolute_rmw!(inc);
absolute_x_rmw!(inc);

fn inx(state: &mut State) {
	state.cpu.x+=1;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) >> 7!=0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn iny(state: &mut State) {
	state.cpu.y+=1;
	state.cpu.p.set_z( 0 == state.cpu.y);
	state.cpu.p.set_n((state.cpu.y & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

fn jmp_absolute(state: &mut State, adr: u16) {
	state.cpu.pc = adr;
	state_step_ppu_many(state, 3);
}

fn jmp_indirect(state: &mut State, adr: u16) {
	let low  = state.get_mem(adr);
	let hi = state.get_mem(adr + 1);
	state.cpu.pc      = u16::from_le_bytes([low, hi]);
	state_step_ppu_many(state, 5);
}

fn jsr(state: &mut State, adr: u16) {
	let return_adr = state.cpu.pc + 2;
	let mut stack_ptr = state.cpu.s;

	let high_byte = (return_adr >> 8) & 0xFF;
	state.set_mem(0x100 + stack_ptr, high_byte);
	stack_ptr-= 1;

	let low_byte = return_adr & 0xFF;
	state.set_mem(0x100 + stack_ptr, low_byte);
	stack_ptr-=1;

	state.cpu.s = stack_ptr;

	state.cpu.pc = adr;
	state_step_ppu_many(state, 6);
}

fn lda_immediate(state: &mut State, val: u8) {
	state.cpu.a   = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 2);
}

fn lda_zero_page(state: &mut State, offset: u8) {
	let val    = state.get_mem(offset as u16);
	state.cpu.a   = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 3);
}

fn lda_zero_page_x(state: &mut State, offset: u8) {
	let val  = state.get_mem((state.cpu.x as u16 + offset as u16) & 0x00FF);
	state.cpu.a = val;
	state.cpu.p.set_z (0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

fn lda_absolute(state: &mut State, adr: u16) {
	state.step_ppu();
	state.step_ppu();
	state.step_ppu();
	let val    = state.get_mem(adr);
	state.cpu.a   = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 3;
	state.step_ppu();
	state.check_interrupt();
}

fn lda_absolute_x(state: &mut State, adr: u16) {
	let res   = state.cpu.x as u16 + adr;
	let val    = state.get_mem(res);
	state.cpu.a   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.a);
	state.cpu.p.N = (uint8_t) ((state.cpu.a & 0x80) >> 7);
	state.cpu.pc += 3;
	let crossed = (res & 0xFF00) == (adr & 0xFF00);
	state_step_ppu_many(if crossed {4} else {5});
}

fn lda_absolute_y(state: &mut State, adr: u16) {
	uint16_t res   = state.cpu.y + adr;
	uint8_t val    = state_get_mem(state, res);
	state.cpu.a   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.a);
	state.cpu.p.N = (uint8_t) ((state.cpu.a & 0x80) >> 7);
	state.cpu.pc += 3;
	bool crossed = (res & 0xFF00) == (adr & 0xFF00);
	state_step_ppu_many(if crossed {4} else {5});
}

STATIC_INLINE void lda_indirect_x(State *state, uint8_t adr) {
	uint8_t tmp    = state_get_mem(state, (uint16_t) (state.cpu.x + adr) & 0xFF);
	uint16_t adr2  = (uint16_t) (state_get_mem(state, (uint16_t) tmp)
				    | state_get_mem(state, (uint16_t) (tmp + 1) & 0xFF) << 8);
	uint8_t val    = state_get_mem(state, adr2);
	state.cpu.a   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.a);
	state.cpu.p.N = (uint8_t) ((state.cpu.a & 0x80) >> 7);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 6);
};

STATIC_INLINE void lda_indirect_y(State *state, uint8_t adr) {
	uint16_t base = (uint16_t) (state_get_mem(state, (uint16_t) adr)
				    | (state_get_mem(state, (uint16_t) ((adr + 1) & 0xFF)) << 8));
	uint16_t adr2 = base + (uint16_t) state.cpu.y;
	uint8_t val   = state_get_mem(state, adr2);

	state.cpu.a   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.a);
	state.cpu.p.N = (uint8_t) ((state.cpu.a & 0x80) >> 7);
	state.cpu.pc += 2;

	bool page_crossed = (adr2 & 0xFF00) != (base & 0xFF00);
	state_step_ppu_many(state, page_crossed ? 6 : 5);
}

STATIC_INLINE void ldx_immediate(State *state, uint8_t val) {
	state_step_ppu_many(state, 2);
	state.cpu.x   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.x);
	state.cpu.p.N = (uint8_t) ((state.cpu.x & 0x80) >> 7);
	state.cpu.pc += 2;
}

STATIC_INLINE void ldx_zero_page(State *state, uint8_t offset) {
	uint8_t val    = state_get_mem(state, (uint16_t) offset);
	state.cpu.x   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.x);
	state.cpu.p.N = (uint8_t) ((state.cpu.x & 0x80) >> 7);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void ldx_zero_page_y(State *state, uint8_t offset) {
	uint8_t val  = state_get_mem(state, ((uint16_t) state.cpu.y + (uint16_t) offset) & 0xFF);
	state.cpu.x = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.x);
	state.cpu.p.N = (uint8_t) ((state.cpu.x & 0x80) >> 7);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ldx_absolute(State *state, uint16_t adr) {
	state_step_ppu_many(state, 3);
	uint8_t val    = state_get_mem(state, adr);
	state.cpu.x   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.x);
	state.cpu.p.N = (uint8_t) ((state.cpu.x & 0x80) >> 7);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void ldx_absolute_y(State *state, uint16_t adr) {
	uint8_t val    = state_get_mem(state, (uint16_t) state.cpu.y + adr);
	state.cpu.x   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.x);
	state.cpu.p.N = (uint8_t) ((state.cpu.x & 0x80) >> 7);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4);
}

[[clang::always_inline]] static inline void ldy_impl(State *state, uint8_t val) {
	state.cpu.y   = val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.y);
	state.cpu.p.N = (uint8_t) ((state.cpu.y & 0x80) >> 7);
}

IMMEDIATE(ldy);
ZERO_PAGE(ldy);
ZERO_PAGE_X(ldy);
ABSOLUTE(ldy);
ABSOLUTE_X(ldy);

[[clang::always_inline]] static inline void lsr_impl(State *state, uint8_t *val) {
	state.cpu.p.C = (uint8_t) (*val & 0x01);
	*val >>= 1;
	state.cpu.p.Z = (uint8_t) (0 == *val);
	state.cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(lsr);
ZERO_PAGE_RMW(lsr);
ZERO_PAGE_X_RMW(lsr);
ABSOLUTE_RMW(lsr);
ABSOLUTE_X_RMW(lsr);

[[clang::always_inline]] static inline void ora_impl(State *state, uint8_t val) {
	state.cpu.a |= val;
	state.cpu.p.Z = (uint8_t) (0 == state.cpu.a);
	state.cpu.p.N = (uint8_t) ((state.cpu.a & 0x80) >> 7);
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
	state_set_mem(state, (uint16_t) (state.cpu.s + 0x100), state.cpu.a);
	state.cpu.s -= 1;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void php(State *state) {
	uint8_t val = state.cpu.p.raw | 0b00110000;
	state_set_mem(state, (uint16_t) (state.cpu.s + 0x100), val);
	state.cpu.s -= 1;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 3);
}

STATIC_INLINE void pla(State *state) {
	state.cpu.s += 1;
	state.cpu.a = state_get_mem(state, (uint16_t) (state.cpu.s + 0x100));
	state.cpu.pc += 1;
	state.cpu.p.Z = 0 == state.cpu.a;
	state.cpu.p.N = (state.cpu.a & 0x80) >> 7;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void plp(State *state) {
	state.cpu.s += 1;
	state.cpu.p.raw = state_get_mem(state, (uint16_t) (state.cpu.s + 0x100));
	state.cpu.pc += 1;
	state_step_ppu_many(state, 4);
}

[[clang::always_inline]] static inline void rol_impl(State *state, uint8_t *val) {
	uint8_t carry  = state.cpu.p.C;
	state.cpu.p.C = (uint8_t) ((*val & 0x80) >> 7);
	*val           = (uint8_t) ((*val << 1) | carry);
	state.cpu.p.Z = (uint8_t) (0 == *val);
	state.cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(rol);
ZERO_PAGE_RMW(rol);
ZERO_PAGE_X_RMW(rol);
ABSOLUTE_RMW(rol);
ABSOLUTE_X_RMW(rol);

[[clang::always_inline]] static inline void ror_impl(State *state, uint8_t *val) {
	uint8_t carry  = state.cpu.p.C;
	state.cpu.p.C = (uint8_t) (*val & 0x01);
	*val           = (uint8_t) ((carry << 7) | (*val >> 1));
	state.cpu.p.Z = (uint8_t) (0 == *val);
	state.cpu.p.N = (uint8_t) ((*val & 0x80) >> 7);
}

ACCUMULATOR(ror);
ZERO_PAGE_RMW(ror);
ZERO_PAGE_X_RMW(ror);
ABSOLUTE_RMW(ror);
ABSOLUTE_X_RMW(ror);

[[clang::always_inline]] static inline void sbc_impl(State *state, uint8_t val) {
	uint16_t res = (uint16_t) state.cpu.a - (uint16_t) val - (uint16_t) (1 - state.cpu.p.C);
	uint16_t a   = state.cpu.a;
	uint16_t val16 = val;

	state.cpu.p.C = res < 256;
	state.cpu.p.Z = 0 == (uint8_t) res;
	state.cpu.p.V = ((res ^ a) & (res ^ ~val16) & 0x80) != 0;
	state.cpu.p.N = (res & 0x80) >> 7;
	state.cpu.a   = (uint8_t) res;
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
	state.cpu.p.C = 1;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sed(State *state) {
	state.cpu.p.D = 1;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sei(State *state) {
	state.cpu.p.I = 1;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void sta_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state.cpu.a);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void sta_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state.cpu.x + (uint16_t) offset) & 0xFF, state.cpu.a);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sta_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state.cpu.a);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sta_absolute_x(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state.cpu.x + adr, state.cpu.a);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

STATIC_INLINE void sta_absolute_y(State *state, uint16_t adr) {
	state_set_mem(state, (uint16_t) state.cpu.y + adr, state.cpu.a);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 5);
};

STATIC_INLINE void sta_indirect_x(State *state, uint8_t adr) {
	uint8_t zp    = (adr + state.cpu.x) & 0xFF;
	uint8_t lo    = state_get_mem(state, zp);
	uint8_t hi    = state_get_mem(state, (zp + 1) & 0xFF);
	uint16_t addr = (uint16_t) (lo | (hi << 8));
	state_set_mem(state, addr, state.cpu.a);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void sta_indirect_y(State *state, uint8_t adr) {
	uint8_t lo    = state_get_mem(state, adr);
	uint8_t hi    = state_get_mem(state, (adr + 1) & 0xFF);
	uint16_t base = (uint16_t) (lo | (hi << 8));
	uint16_t addr = base + state.cpu.y;
	state_set_mem(state, addr, state.cpu.a);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void stx_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state.cpu.x);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void stx_zero_page_y(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state.cpu.y + (uint16_t) offset) & 0xFF, state.cpu.x);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void stx_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state.cpu.x);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sty_zero_page(State *state, uint8_t offset) {
	state_set_mem(state, (uint16_t) offset, state.cpu.y);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 3);
};

STATIC_INLINE void sty_zero_page_x(State *state, uint8_t offset) {
	state_set_mem(state, ((uint16_t) state.cpu.x + (uint16_t) offset) & 0xFF, state.cpu.y);
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void sty_absolute(State *state, uint16_t adr) {
	state_set_mem(state, adr, state.cpu.y);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4);
};

STATIC_INLINE void tax(State *state) {
	state.cpu.x   = state.cpu.a;
	state.cpu.p.Z = 0 == state.cpu.x;
	state.cpu.p.N = (state.cpu.x & 0x80) >> 7;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tay(State *state) {
	state.cpu.y   = state.cpu.a;
	state.cpu.p.Z = 0 == state.cpu.y;
	state.cpu.p.N = (state.cpu.y & 0x80) >> 7;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tsx(State *state) {
	state.cpu.x   = state.cpu.s;
	state.cpu.p.Z = 0 == state.cpu.x;
	state.cpu.p.N = (state.cpu.x & 0x80) >> 7;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void txa(State *state) {
	state.cpu.a   = state.cpu.x;
	state.cpu.p.Z = 0 == state.cpu.a;
	state.cpu.p.N = (state.cpu.a & 0x80) >> 7;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void txs(State *state) {
	state.cpu.s = state.cpu.x;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void tya(State *state) {
	state.cpu.a   = state.cpu.y;
	state.cpu.p.Z = 0 == state.cpu.a;
	state.cpu.p.N = (state.cpu.y & 0x80) >> 7;
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void rti(State *state) {
	state.cpu.s += 1;
	state.cpu.p.raw = state_get_mem(state, (uint16_t) (state.cpu.s + 0x100));
	state.cpu.s += 2;
	state.cpu.pc =
	    (uint16_t) (state_get_mem(state, (uint16_t) (state.cpu.s + 0x100 - 1))
			| state_get_mem(state, (uint16_t) (state.cpu.s + 0x100)) << 8);
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void rts(State *state) {
	state.cpu.s += 2;
	state.cpu.pc =
	    (uint16_t) ((state_get_mem(state, (uint16_t) (state.cpu.s + 0x100 - 1))
			 | state_get_mem(state, (uint16_t) (state.cpu.s + 0x100)) << 8)
			+ 1);
	state_step_ppu_many(state, 6);
}

STATIC_INLINE void nop(State *state) {
	state.cpu.pc += 1;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void skb(State *state) {
	state.cpu.pc += 2;
	state_step_ppu_many(state, 2);
}

STATIC_INLINE void ign(State *state, uint16_t) {
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_direct(State *state, uint8_t) {
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_direct_x(State *state, uint8_t) {
	state.cpu.pc += 2;
	state_step_ppu_many(state, 4);
}

STATIC_INLINE void ign_absolute_x(State *state, uint16_t adr) {
	uint16_t actual_adr = (uint16_t) state.cpu.x + adr;
	bool page_crossed   = state.cpu.x + (adr & 0xFF) > 0xFF;
	(void) state_get_mem(state, actual_adr);
	state.cpu.pc += 3;
	state_step_ppu_many(state, 4 + page_crossed);
}

STATIC_INLINE void lax_immediate(State *state, uint8_t) {
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_zero_page(State *state, uint8_t val) {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.Z = (val == 0);
	state.cpu.p.N = ((val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_zero_page_y(State *state, uint8_t val) {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.Z = (val == 0);
	state.cpu.p.N = ((val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_absolute(State *state, uint16_t val) {
	state.cpu.a = (uint8_t) val;
	state.cpu.x = (uint8_t) val;
	// Update flags
	state.cpu.p.Z = ((uint8_t) val == 0);
	state.cpu.p.N = (((uint8_t) val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_absolute_y(State *state, uint16_t val) {
	state.cpu.a = (uint8_t) val;
	state.cpu.x = (uint8_t) val;
	// Update flags
	state.cpu.p.Z = (val == 0);
	state.cpu.p.N = ((val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_indirect_x(State *state, uint8_t val) {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.Z = (val == 0);
	state.cpu.p.N = ((val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void lax_indirect_y(State *state, uint8_t val) {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.Z = (val == 0);
	state.cpu.p.N = ((val & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void sax_zero_page(State *state, uint8_t val) {
	uint8_t result = state.cpu.a & state.cpu.x;
	state_set_mem(state, val, result);
	state.cpu.pc += 2;
}

STATIC_INLINE void sax_zero_page_y(State *state, uint8_t val) {
	uint8_t result = state.cpu.a & state.cpu.x;
	state_set_mem(state, val, result);
	state.cpu.pc += 2;
}

STATIC_INLINE void sax_absolute(State *state, uint16_t val) {
	uint8_t result = state.cpu.a & state.cpu.x;
	state_set_mem(state, (uint8_t) val, result);
	state.cpu.pc += 2;
}

STATIC_INLINE void sax_indirect_x(State *state, uint8_t val) {
	uint8_t result = state.cpu.a & state.cpu.x;
	state_set_mem(state, val, result);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_zero_page(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_zero_page_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val - 1;
	state_set_mem(state, (uint8_t) val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute_x(State *state, uint16_t val) {
	uint8_t result = (uint8_t)val - 1;
	state_set_mem(state, (uint8_t)val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_absolute_y(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val - 1;
	state_set_mem(state, (uint8_t) val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_indirect_x(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void dcp_indirect_y(State *state, uint8_t val) {
	uint8_t result = val - 1;
	state_set_mem(state, val, result);

	// Compare
	uint8_t temp   = state.cpu.a - result;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_zero_page(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_zero_page_x(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_absolute(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, (uint8_t) val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_absolute_x(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_absolute_y(State *state, uint16_t val) {
	uint8_t result = (uint8_t) val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_indirect_x(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void isc_indirect_y(State *state, uint8_t val) {
	uint8_t result = val + 1;
	state_set_mem(state, val, result);

	// Subtract with borrow
	uint8_t temp   = state.cpu.a - result - (1 - state.cpu.p.C);
	state.cpu.p.C = (temp <= state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);
	state.cpu.a   = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_zero_page(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_zero_page_x(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_absolute(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_absolute_x(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_absolute_y(State *state, uint16_t val) {
	// Rotate left
	uint8_t carry  = ((uint8_t) val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_indirect_x(State *state, uint8_t val) {
	// Rotate left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rla_indirect_y(State *state, uint8_t val) {
	// Rotate left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) ((val << 1) | (uint8_t) state.cpu.p.C);
	state.cpu.p.C = carry;

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_zero_page(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_zero_page_x(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_absolute(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_absolute_x(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_absolute_y(State *state, uint16_t val) {
	// Rotate right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_indirect_x(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void rra_indirect_y(State *state, uint8_t val) {
	// Rotate right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) ((val >> 1) | ((uint8_t) state.cpu.p.C << 7));
	state.cpu.p.C = carry;

	// Add with carry
	uint8_t temp   = state.cpu.a + result + state.cpu.p.C;
	state.cpu.p.C = (temp < state.cpu.a);
	state.cpu.p.Z = (temp == 0);
	state.cpu.p.N = ((temp & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_zero_page(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_zero_page_x(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_absolute(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_absolute_x(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_absolute_y(State *state, uint16_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_indirect_x(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void slo_indirect_y(State *state, uint8_t val) {
	// Shift left
	uint8_t carry  = (val & 0x80) ? 1 : 0;
	uint8_t result = (uint8_t) (val << 1);
	state.cpu.p.C = carry;

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_zero_page(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_zero_page_x(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_absolute(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_absolute_x(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_absolute_y(State *state, uint16_t val) {
	// Shift right
	uint8_t carry  = ((uint8_t) val & 0x01) ? 0x80 : 0;
	uint8_t result = (uint8_t) val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, (uint8_t) val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_indirect_x(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void sre_indirect_y(State *state, uint8_t val) {
	// Shift right
	uint8_t carry  = (val & 0x01) ? 0x80 : 0;
	uint8_t result = val >> 1;
	state.cpu.p.C = carry;

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	state_set_mem(state, val, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void anc_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state.cpu.a &= val;

	// Update flags
	state.cpu.p.Z = (state.cpu.a == 0);
	state.cpu.p.N = ((state.cpu.a & 0x80) != 0);

	// Set carry flag to bit 7 of accumulator
	state.cpu.p.C = ((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void alr_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state.cpu.a &= val;

	// Shift right
	uint8_t carry = (state.cpu.a & 0x01) ? 0x80 : 0;
	state.cpu.a >>= 1;
	state.cpu.p.C = carry;

	// Update flags
	state.cpu.p.Z = (state.cpu.a == 0);
	state.cpu.p.N = ((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void arr_immediate(State *state, uint8_t val) {
	// AND with accumulator
	state.cpu.a &= val;

	// Shift right with carry
	uint8_t carry = (state.cpu.a & 0x01) ? 0x80 : 0;
	state.cpu.a >>= 1;
	state.cpu.a |= ((uint8_t) state.cpu.p.C << 7);
	state.cpu.p.C = carry;

	// Update flags
	state.cpu.p.Z = (state.cpu.a == 0);
	state.cpu.p.N = ((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void axs_immediate(State *state, uint8_t val) {
	// AND accumulator with X register
	uint8_t temp = state.cpu.a & state.cpu.x;

	// Subtract with borrow
	uint8_t result = temp - val - (1 - state.cpu.p.C);
	state.cpu.p.C = (result <= temp);
	state.cpu.p.Z = (result == 0);
	state.cpu.p.N = ((result & 0x80) != 0);

	// Store result in X register
	state.cpu.x = result;
	state.cpu.pc += 2;
}

STATIC_INLINE void las_immediate(State *state, uint8_t val) {
	// AND with accumulator and store in A, X, and S
	state.cpu.a &= val;
	state.cpu.x = state.cpu.a;
	state.cpu.s = state.cpu.a;

	// Update flags
	state.cpu.p.Z = (state.cpu.a == 0);
	state.cpu.p.N = ((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
}

STATIC_INLINE void tas_immediate(State *state, uint8_t val) {
	// AND accumulator with X register and store in S
	uint8_t temp = state.cpu.a & state.cpu.x;
	state.cpu.s = temp;

	// Store S in memory
	state_set_mem(state, val, temp);
	state.cpu.pc += 2;
}

STATIC_INLINE void tas_absolute_y(State *state, uint16_t) {
	state.cpu.pc += 3;
}

STATIC_INLINE void shy_immediate(State *state, uint8_t val) {
	// Store Y register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store Y register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state.cpu.y);
	state.cpu.pc += 2;
}

STATIC_INLINE void shy_absolute_x(State *state, uint16_t) {
	state.cpu.pc += 3;
}

STATIC_INLINE void shx_immediate(State *state, uint8_t val) {
	// Store X register with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store X register in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state.cpu.x);
	state.cpu.pc += 2;
}

STATIC_INLINE void shx_absolute_y(State *state, uint16_t) {
	state.cpu.pc += 3;
}

STATIC_INLINE void ahx_absolute_y(State *state, uint16_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = (uint8_t) val;
	uint8_t adr_high = ((uint8_t) val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state.cpu.a & state.cpu.x);
	state.cpu.pc += 2;
}

STATIC_INLINE void ahx_indirect_y(State *state, uint8_t val) {
	// Store A and X registers with high byte of adress
	uint8_t adr_low  = val;
	uint8_t adr_high = (val + 1) & 0xFF;

	// Store (A & X) in memory
	state_set_mem(state, (uint16_t) (adr_low | (adr_high << 8)), state.cpu.a & state.cpu.x);
	state.cpu.pc += 2;
}

STATIC_INLINE void stp(State *state) {
	state.cpu.pc += 1;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void xaa_immediate(State *state, uint8_t) {
	state.cpu.pc += 2;
	state_step_ppu_many(state, 1);
}

STATIC_INLINE void las_absolute_y(State *state, uint16_t) {
	state.cpu.pc += 2;
	state_step_ppu_many(state, 1);
}
