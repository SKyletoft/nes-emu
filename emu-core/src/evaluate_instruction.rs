#![allow(unused)]

use paste::paste;

use crate::{
	interpret::{State, StateTail},
	mapper::Mapper,
};

#[inline(always)]
fn advance<M: Mapper, F>(tail: &mut StateTail<M, F>, by: usize) {
	tail.cycles += by;
	tail.ppu_runahead += by * 3;
}

macro_rules! accumulator {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _accumulator>]<M: Mapper, F>(mut state: State<M, F>) -> State<M,F> {
				let mut a = state.cpu.a;
				[<$fn _impl>](&mut state, &mut a);
				state.cpu.a = a;
				state.cpu.pc += 1;
				advance(&mut state.rest, 2);
				state
			}
		}
	};
}

macro_rules! immediate {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _immediate>]<M: Mapper, F>(mut state: State<M,F>, val: u8) -> State<M,F> {
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 2);
				state
			}
		}
	};
}

macro_rules! zero_page {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page>]<M: Mapper, F>(mut state: State<M,F>, offset: u8) -> State<M,F> {
				let val = state.mem(offset as u16);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 3);
				state
			}
		}
	};
}

macro_rules! zero_page_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page>]<M: Mapper, F>(mut state: State<M,F>, offset: u8) -> State<M,F> {
				let mut val = state.mem(offset as u16);
				[<$fn _impl>](&mut state, &mut val);
				state.set_mem(offset as u16, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 5);
				state
			}
		}
	};
}

macro_rules! zero_page_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page_x>]<M: Mapper, F>(mut state: State<M,F>, offset: u8) -> State<M,F> {
				let adr = state.cpu.x.wrapping_add(offset) as u16;
				let val = state.mem(adr & 0x00FF);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 4);
				state
			}
		}
	};
}

macro_rules! zero_page_x_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _zero_page_x>]<M: Mapper, F>(mut state: State<M,F>, offset: u8) -> State<M,F> {
				let actual_adr = (state.cpu.x.wrapping_add(offset)) as u16 & 0x00FF;
				let mut val = state.mem(actual_adr);
				[<$fn _impl>](&mut state, &mut val);
				state.set_mem(actual_adr, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 6);
				state
			}
		}
	};
}

macro_rules! absolute {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute>]<M: Mapper, F>(mut state: State<M,F>, adr: u16) -> State<M,F> {
				let val = state.mem(adr);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 3;
				advance(&mut state.rest, 4);
				state
			}
		}
	};
}

macro_rules! absolute_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute>]<M: Mapper, F>(mut state: State<M,F>, adr: u16) -> State<M,F> {
				let mut val = state.mem(adr);
				[<$fn _impl>](&mut state, &mut val);
				state.set_mem(adr, val);
				state.cpu.pc += 3;
				advance(&mut state.rest, 6);
				state
			}
		}
	};
}

macro_rules! absolute_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_x>]<M: Mapper, F>(mut state: State<M,F>, adr: u16) -> State<M,F> {
				let actual_adr = adr.wrapping_add(state.cpu.x as u16);
				let page_crossed = (state.cpu.x as u16 + (adr & 0x00FF)) > 0x00FF;
				let val = state.mem(actual_adr);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 3;
				advance(&mut state.rest, 4 + page_crossed as usize);
				state
			}
		}
	};
}

macro_rules! absolute_x_rmw {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_x>]<M: Mapper, F>(mut state: State<M,F>, adr: u16) -> State<M,F> {
				let actual_adr = adr.wrapping_add(state.cpu.x as u16);
				let mut val = state.mem(actual_adr);
				[<$fn _impl>](&mut state, &mut val);
				state.set_mem(actual_adr, val);
				state.cpu.pc += 3;
				advance(&mut state.rest, 7);
				state
			}
		}
	};
}

macro_rules! absolute_y {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _absolute_y>]<M: Mapper, F>(mut state: State<M,F>, adr: u16) -> State<M,F> {
				let actual_adr = adr.wrapping_add(state.cpu.y as u16);
				let page_crossed = (state.cpu.y as u16 + (adr & 0x00FF)) > 0x00FF;
				let val = state.mem(actual_adr);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 3;
				advance(&mut state.rest, 4 + page_crossed as usize);
				state
			}
		}
	};
}

macro_rules! indirect_x {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _indirect_x>]<M: Mapper, F>(mut state: State<M,F>, adr: u8) -> State<M,F> {
				let tmp = state.mem(state.cpu.x.wrapping_add(adr) as u16 & 0x00FF);
				let lo = state.mem(tmp as u16);
				let hi = state.mem((tmp.wrapping_add(1)) as u16 & 0x00FF);
				let adr2 = (lo as u16) | ((hi as u16) << 8);
				let val = state.mem(adr2);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 6);
				state
			}
		}
	};
}

macro_rules! indirect_y {
	($fn:ident) => {
		paste! {
			#[inline(always)]
			pub fn [<$fn _indirect_y>]<M: Mapper, F>(mut state: State<M,F>, adr: u8) -> State<M,F> {
				let tmp = state.mem(state.cpu.y.wrapping_add(adr) as u16 & 0x00FF);
				let adr2 = u16::from_le_bytes([
					state.mem(tmp as u16),
					state.mem((tmp.wrapping_add(1)) as u16 & 0x00FF),
				]);
				let taken = (adr2 & 0x00FF) == 0;
				let val = state.mem(adr2);
				[<$fn _impl>](&mut state, val);
				state.cpu.pc += 2;
				advance(&mut state.rest, 5 + taken as usize);
				state
			}
		}
	};
}

#[inline(always)]
fn adc_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	let res = state.cpu.a as u16 + state.cpu.p.c() as u16 + val as u16;
	state.cpu.p.set_c(res > 0xFF);
	state.cpu.p.set_z((res as u8) == 0);
	state
		.cpu
		.p
		.set_v(((res ^ state.cpu.a as u16) & (res ^ val as u16) & 0x80) != 0);
	state.cpu.p.set_n(res & 0x80 != 0);
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

#[inline(always)]
fn and_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
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

#[inline(always)]
fn asl_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	state.cpu.p.set_c(*val & 0x80 != 0);
	*val <<= 1;
	state.cpu.p.set_z(*val == 0);
	state.cpu.p.set_n(*val & 0x80 != 0);
}

accumulator!(asl);
zero_page_rmw!(asl);
zero_page_x_rmw!(asl);
absolute_rmw!(asl);
absolute_x_rmw!(asl);

#[inline(always)]
pub fn bcs<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = state.cpu.p.c();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bcc<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = !state.cpu.p.c();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn beq<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = state.cpu.p.z();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bne<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = !state.cpu.p.z();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bmi<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = state.cpu.p.n();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bpl<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = !state.cpu.p.n();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != new_pc & 0xFF00;
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bvs<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = state.cpu.p.v();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
pub fn bvc<M: Mapper, F>(mut state: State<M, F>, offset: i8) -> State<M, F> {
	let old_pc = state.cpu.pc;
	let taken = !state.cpu.p.v();
	let new_pc = old_pc + 2 + if taken { offset as u16 } else { 0 };
	let page_crossed = (old_pc + 2) & 0xFF00 != (new_pc & 0xFF00);
	let cycles = 2 + taken as usize + page_crossed as usize;
	state.cpu.pc = new_pc;
	advance(&mut state.rest, cycles);
	state
}

#[inline(always)]
fn bit_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	state.cpu.p.set_z(state.cpu.a & val == 0);
	state.cpu.p.set_v((val & 0x40) != 0);
	state.cpu.p.set_n((val & 0x80) != 0);
}

zero_page!(bit);
absolute!(bit);

#[inline(always)]
pub fn brk<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn clc<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_c(false);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn cld<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_d(false);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn cli<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_i(false);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn clv<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_v(false);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
fn cmp_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	let res = state.cpu.a as u16 - val as u16;
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

#[inline(always)]
fn cpx_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	let res = state.cpu.x as u16 - val as u16;
	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state.cpu.p.set_n(res & 0x80 != 0);
}

immediate!(cpx);
zero_page!(cpx);
absolute!(cpx);

#[inline(always)]
fn cpy_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	let res = state.cpu.y as u16 - val as u16;
	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state.cpu.p.set_n(res & 0x80 != 0);
}

immediate!(cpy);
zero_page!(cpy);
absolute!(cpy);

#[inline(always)]
fn dec_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	*val -= 1;
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) != 0);
}

zero_page_rmw!(dec);
zero_page_x_rmw!(dec);
absolute_rmw!(dec);
absolute_x_rmw!(dec);

#[inline(always)]
pub fn dex<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.x -= 1;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn dey<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.y -= 1;
	state.cpu.p.set_z(0 == state.cpu.y);
	state.cpu.p.set_n((state.cpu.y & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
fn eor_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
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

#[inline(always)]
fn inc_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	*val += 1;
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) >> 7 != 0);
}

zero_page_rmw!(inc);
zero_page_x_rmw!(inc);
absolute_rmw!(inc);
absolute_x_rmw!(inc);

#[inline(always)]
pub fn inx<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.x += 1;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn iny<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.y += 1;
	state.cpu.p.set_z(0 == state.cpu.y);
	state.cpu.p.set_n((state.cpu.y & 0x80) >> 7 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn jmp_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	// if adr == state.cpu.pc {
	//	state.wait_for_interrupt();
	// } else {
		state.cpu.pc = adr;
		advance(&mut state.rest, 3);
	// }
	state
}

#[inline(always)]
pub fn jmp_indirect<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let low = state.mem(adr);
	let hi = state.mem(adr + 1);
	let target_adr = u16::from_le_bytes([low, hi]);
	// if target_adr == state.cpu.pc {
	//	state.wait_for_interrupt();
	// } else {
		state.cpu.pc = target_adr;
		advance(&mut state.rest, 5);
	// }
	state
}

#[inline(always)]
pub fn jsr<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let return_adr = state.cpu.pc + 2;
	let mut stack_ptr = state.cpu.s;

	let [lo, hi] = return_adr.to_le_bytes();
	state.set_mem(0x100 + stack_ptr as u16, hi);
	stack_ptr -= 1;

	state.set_mem(0x100 + stack_ptr as u16, lo);
	stack_ptr -= 1;

	state.cpu.s = stack_ptr;

	state.cpu.pc = adr;
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn lda_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n(state.cpu.a & 0x80 != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn lda_zero_page<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	let val = state.mem(offset as u16);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn lda_zero_page_x<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	let val = state.mem((state.cpu.x as u16 + offset as u16) & 0x00FF);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(never)]
pub fn lda_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	// LDA Absolute *really* cares about timing,
	// so actually catch up and step ppu here
	state.rest.ppu_runahead += 9;
	#[cfg(test)]
	state.catch_up_ppu();
	let val = state.mem(adr);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 3;
	state.rest.cycles += 4;
	state.rest.ppu_runahead += 3;
	state.check_interrupt();
	state
}

#[inline(always)]
pub fn lda_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let res = state.cpu.x as u16 + adr;
	let val = state.mem(res);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n(state.cpu.a & 0x80 != 0);
	state.cpu.pc += 3;
	let crossed = (res & 0xFF00) == (adr & 0xFF00);
	advance(&mut state.rest, if crossed { 4 } else { 5 });
	state
}

#[inline(always)]
pub fn lda_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let res = state.cpu.y as u16 + adr;
	let val = state.mem(res);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 3;
	let crossed = (res & 0xFF00) == (adr & 0xFF00);
	advance(&mut state.rest, if crossed { 4 } else { 5 });
	state
}

#[inline(always)]
pub fn lda_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let tmp = state.mem(state.cpu.x.wrapping_add(adr) as u16);
	let adr2 = u16::from_le_bytes([state.mem(tmp as u16), state.mem(tmp.wrapping_add(1) as u16)]);
	let val = state.mem(adr2);
	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) >> 7 != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn lda_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let base = u16::from_le_bytes([state.mem(adr as u16), state.mem(adr.wrapping_add(1) as u16)]);
	let adr2 = base + state.cpu.y as u16;
	let val = state.mem(adr2);

	state.cpu.a = val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;

	let page_crossed = (adr2 & 0xFF00) != (base & 0xFF00);
	advance(&mut state.rest, if page_crossed { 6 } else { 5 });
	state
}

#[inline(always)]
pub fn ldx_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.x = val;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn ldx_zero_page<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	let val = state.mem(offset as u16);
	state.cpu.x = val;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn ldx_zero_page_y<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	let val = state.mem(state.cpu.y.wrapping_add(offset) as u16);
	state.cpu.x = val;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) != 0);
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn ldx_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	// Same as LDA Absolute, timing sensitive
	state.rest.ppu_runahead += 9;
	#[cfg(test)]
	state.catch_up_ppu();
	let val = state.mem(adr);
	state.cpu.x = val;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) >> 7 != 0);
	state.cpu.pc += 3;
	state.rest.cycles += 4;
	state.rest.ppu_runahead += 3;
	state.check_interrupt();
	state
}

#[inline(always)]
pub fn ldx_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(state.cpu.y as u16 + adr);
	state.cpu.x = val;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n((state.cpu.x & 0x80) != 0);
	state.cpu.pc += 3;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
fn ldy_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	state.cpu.y = val;
	state.cpu.p.set_z(0 == state.cpu.y);
	state.cpu.p.set_n((state.cpu.y & 0x80) != 0);
}

immediate!(ldy);
zero_page!(ldy);
zero_page_x!(ldy);
absolute!(ldy);
absolute_x!(ldy);

#[inline(always)]
fn lsr_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	state.cpu.p.set_c(*val & 0x01 != 0);
	*val >>= 1;
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) != 0);
}

accumulator!(lsr);
zero_page_rmw!(lsr);
zero_page_x_rmw!(lsr);
absolute_rmw!(lsr);
absolute_x_rmw!(lsr);

#[inline(always)]
fn ora_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	state.cpu.a |= val;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);
}

immediate!(ora);
zero_page!(ora);
zero_page_x!(ora);
absolute!(ora);
absolute_x!(ora);
absolute_y!(ora);
indirect_x!(ora);
indirect_y!(ora);

#[inline(always)]
pub fn pha<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.set_mem(state.cpu.s as u16 + 0x100, state.cpu.a);
	state.cpu.s -= 1;
	state.cpu.pc += 1;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn php<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	let val = state.cpu.p.into_bits() | 0b00110000;
	state.set_mem((state.cpu.s as u16 + 0x100), val);
	state.cpu.s -= 1;
	state.cpu.pc += 1;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn pla<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.s += 1;
	state.cpu.a = state.mem(state.cpu.s as u16 + 0x100);
	state.cpu.pc += 1;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n(state.cpu.a & 0x80 != 0);
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn plp<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.s += 1;
	let cc = state.mem(state.cpu.s as u16 + 0x100);
	state.cpu.p.set_bits(cc);
	state.cpu.pc += 1;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
fn rol_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	let carry = state.cpu.p.c();
	state.cpu.p.set_c((*val & 0x80) != 0);
	*val = ((*val << 1) | carry as u8);
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) != 0);
}

accumulator!(rol);
zero_page_rmw!(rol);
zero_page_x_rmw!(rol);
absolute_rmw!(rol);
absolute_x_rmw!(rol);

#[inline(always)]
fn ror_impl<M: Mapper, F>(state: &mut State<M, F>, val: &mut u8) {
	let carry = state.cpu.p.c();
	state.cpu.p.set_c(*val & 0x01 != 0);
	*val = (carry as u8) << 7 | *val >> 1;
	state.cpu.p.set_z(0 == *val);
	state.cpu.p.set_n((*val & 0x80) != 0);
}

accumulator!(ror);
zero_page_rmw!(ror);
zero_page_x_rmw!(ror);
absolute_rmw!(ror);
absolute_x_rmw!(ror);

#[inline(always)]
fn sbc_impl<M: Mapper, F>(state: &mut State<M, F>, val: u8) {
	let res = (state.cpu.a as u16)
		.wrapping_sub(val as u16)
		.wrapping_sub(!state.cpu.p.c() as u16);

	state.cpu.p.set_c(res < 256);
	state.cpu.p.set_z(0 == res as u8);
	state
		.cpu
		.p
		.set_v((res as u8 ^ state.cpu.a) & (res as u8 ^ !val) & 0x80 != 0);
	state.cpu.p.set_n(res & 0x80 != 0);
	state.cpu.a = res as u8;
}

immediate!(sbc);
zero_page!(sbc);
zero_page_x!(sbc);
absolute!(sbc);
absolute_x!(sbc);
absolute_y!(sbc);
indirect_x!(sbc);
indirect_y!(sbc);

#[inline(always)]
pub fn sec<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_c(true);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn sed<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_d(true);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn sei<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.p.set_i(true);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn sta_zero_page<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(offset as u16, state.cpu.a);
	state.cpu.pc += 2;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn sta_zero_page_x<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(state.cpu.x.wrapping_add(offset) as u16, state.cpu.a);
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn sta_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(adr, state.cpu.a);
	state.cpu.pc += 3;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn sta_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(state.cpu.x as u16 + adr, state.cpu.a);
	state.cpu.pc += 3;
	advance(&mut state.rest, 5);
	state
}

#[inline(always)]
pub fn sta_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(state.cpu.y as u16 + adr, state.cpu.a);
	state.cpu.pc += 3;
	advance(&mut state.rest, 5);
	state
}

#[inline(always)]
pub fn sta_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let zp = (adr + state.cpu.x);
	let lo = state.mem(zp as u16);
	let hi = state.mem(zp.wrapping_add(1) as u16);
	let adr = u16::from_le_bytes([lo, hi]);
	state.set_mem(adr, state.cpu.a);
	state.cpu.pc += 2;
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn sta_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let lo = state.mem(adr as u16);
	let hi = state.mem(adr.wrapping_add(1) as u16);
	let base = u16::from_le_bytes([lo, hi]);
	let adr = base + state.cpu.y as u16;
	state.set_mem(adr, state.cpu.a);
	state.cpu.pc += 2;
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn stx_zero_page<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(offset as u16, state.cpu.x);
	state.cpu.pc += 2;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn stx_zero_page_y<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(state.cpu.y.wrapping_add(offset) as u16, state.cpu.x);
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn stx_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(adr, state.cpu.x);
	state.cpu.pc += 3;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn sty_zero_page<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(offset as u16, state.cpu.y);
	state.cpu.pc += 2;
	advance(&mut state.rest, 3);
	state
}

#[inline(always)]
pub fn sty_zero_page_x<M: Mapper, F>(mut state: State<M, F>, offset: u8) -> State<M, F> {
	state.set_mem(state.cpu.x.wrapping_add(offset) as u16, state.cpu.y);
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn sty_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(adr, state.cpu.y);
	state.cpu.pc += 3;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn tax<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.x = state.cpu.a;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n(state.cpu.x & 0x80 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn tay<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.y = state.cpu.a;
	state.cpu.p.set_z(0 == state.cpu.y);
	state.cpu.p.set_n(state.cpu.y & 0x80 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn tsx<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.x = state.cpu.s;
	state.cpu.p.set_z(0 == state.cpu.x);
	state.cpu.p.set_n(state.cpu.x & 0x80 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn txa<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.a = state.cpu.x;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n(state.cpu.a & 0x80 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn txs<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.s = state.cpu.x;
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn tya<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.a = state.cpu.y;
	state.cpu.p.set_z(0 == state.cpu.a);
	state.cpu.p.set_n(state.cpu.y & 0x80 != 0);
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn rti<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.s += 1;
	let cc = state.mem(state.cpu.s as u16 + 0x100);
	state.cpu.p.set_bits(cc);
	state.cpu.s += 2;
	state.cpu.pc = u16::from_le_bytes([
		state.mem(state.cpu.s as u16 + 0x100 - 1),
		state.mem(state.cpu.s as u16 + 0x100),
	]);
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn rts<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.s += 2;
	state.cpu.pc = u16::from_le_bytes([
		state.mem(state.cpu.s as u16 + 0x100 - 1),
		state.mem(state.cpu.s as u16 + 0x100),
	]) + 1;
	advance(&mut state.rest, 6);
	state
}

#[inline(always)]
pub fn nop<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.pc += 1;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn skb<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.pc += 2;
	advance(&mut state.rest, 2);
	state
}

#[inline(always)]
pub fn ign<M: Mapper, F>(mut state: State<M, F>, _: u16) -> State<M, F> {
	state.cpu.pc += 3;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn ign_direct<M: Mapper, F>(mut state: State<M, F>, _: u8) -> State<M, F> {
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn ign_direct_x<M: Mapper, F>(mut state: State<M, F>, _: u8) -> State<M, F> {
	state.cpu.pc += 2;
	advance(&mut state.rest, 4);
	state
}

#[inline(always)]
pub fn ign_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let actual_adr = state.cpu.x as u16 + adr;
	let page_crossed = state.cpu.x.checked_add(adr as u8).is_none();
	let _ = state.mem(actual_adr);
	state.cpu.pc += 3;
	state.rest.ppu_runahead += (4 + page_crossed as usize);
	state
}

#[inline(always)]
pub fn lax_immediate<M: Mapper, F>(mut state: State<M, F>, _: u8) -> State<M, F> {
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_zero_page<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n((val & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_zero_page_y<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.a = val;
	state.cpu.x = val;
	// Update flags
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n((val & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_absolute<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	state.cpu.a = val as u8;
	state.cpu.x = val as u8;
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n(val & 0x80 != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_absolute_y<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	state.cpu.a = val as u8;
	state.cpu.x = val as u8;
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n(val & 0x80 != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_indirect_x<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.a = val;
	state.cpu.x = val;
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n(val & 0x80 != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn lax_indirect_y<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	state.cpu.a = val;
	state.cpu.x = val;
	state.cpu.p.set_z(val == 0);
	state.cpu.p.set_n(val & 0x80 != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sax_zero_page<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let result = state.cpu.a & state.cpu.x;
	state.set_mem(adr as u16, result);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sax_zero_page_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let result = state.cpu.a & state.cpu.x;
	state.set_mem(adr as u16, result);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sax_absolute<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	let result = state.cpu.a & state.cpu.x;
	state.set_mem(val & 0xFF, result);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sax_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let result = state.cpu.a & state.cpu.x;
	state.set_mem(adr as u16, result);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_zero_page<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	let result = val - 1;
	state.set_mem(val as u16, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_zero_page_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val - 1;
	state.set_mem(adr as u16, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_absolute<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	let result = (val as u8).wrapping_sub(1);
	state.set_mem(val & 0xFF, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_absolute_x<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	let result = (val as u8).wrapping_sub(1);
	state.set_mem(val & 0xFF, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_absolute_y<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	let result = (val as u8).wrapping_sub(1);
	state.set_mem(val & 0xFF, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val - 1;
	state.set_mem(adr as u16, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn dcp_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val - 1;
	state.set_mem(adr as u16, result);

	// Compare
	let temp = state.cpu.a - result;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_zero_page<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val + 1;
	state.set_mem(adr as u16, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_zero_page_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val + 1;
	state.set_mem(adr as u16, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);
	let result = val.wrapping_add(1);
	state.set_mem(adr & 0xFF, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);
	let result = val.wrapping_add(1);
	state.set_mem(adr, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);
	let result = val.wrapping_add(1);
	state.set_mem(adr, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val + 1;
	state.set_mem(adr as u16, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn isc_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	let result = val + 1;
	state.set_mem(adr as u16, result);

	// Subtract with borrow
	let temp = state.cpu.a - result - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(temp <= state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_zero_page<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// Rotate left
	let carry = val & 0x80 != 0;
	let mut result = (val << 1) | state.cpu.p.c() as u8;
	state.cpu.p.set_c(carry);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_zero_page_x<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// Rotate left
	let carry = val & 0x80 != 0;
	let mut result = (val << 1) | state.cpu.p.c() as u8;
	state.cpu.p.set_c(carry);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_absolute<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	// Rotate left
	let carry = val & 0x80 != 0;
	let mut result = (val << 1) as u8 | state.cpu.p.c() as u8;
	state.cpu.p.set_c(carry);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_absolute_x<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	// Rotate left
	let carry = val & 0x80 != 0;
	let mut result = (val << 1) as u8 | state.cpu.p.c() as u8;
	state.cpu.p.set_c(carry);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_absolute_y<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	// Rotate left
	let carry = val & 0x80 != 0;
	let mut result = (val << 1) as u8 | state.cpu.p.c() as u8;
	state.cpu.p.set_c(carry);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	// Rotate left
	let mut result = (val << 1) | state.cpu.p.c() as u8;
	state.cpu.p.set_c((val & 0x80) != 0);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rla_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);
	// Rotate left
	let mut result = (val << 1) | state.cpu.p.c() as u8;
	state.cpu.p.set_c((val & 0x80) != 0);

	// AND with accumulator
	result &= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_zero_page<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_zero_page_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn rra_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Rotate right
	let result = val >> 1 | (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(val & 1 != 0);

	// Add with carry
	let temp = state.cpu.a + result + state.cpu.p.c() as u8;
	state.cpu.p.set_c(temp < state.cpu.a);
	state.cpu.p.set_z(temp == 0);
	state.cpu.p.set_n((temp & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = temp;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_zero_page<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift left
	let mut result = (val << 1);
	state.cpu.p.set_c((val & 0x80) != 0);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_zero_page_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = (val << 1);
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = (val << 1);
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_absolute_x<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = val << 1;
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = val << 1;
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = val << 1;
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn slo_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift left
	let carry = (val & 0x80) != 0;
	let mut result = val << 1;
	state.cpu.p.set_c(carry);

	// OR with accumulator
	result |= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_zero_page<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift right
	let mut result = val >> 1;
	state.cpu.p.set_c(val & 1 != 0);

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_zero_page_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	// Shift right
	let mut result = val >> 1;
	state.cpu.p.set_c(val & 1 != 0);

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_absolute<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	let val = state.mem(adr);

	// Shift right
	let mut result = (val >> 1);
	state.cpu.p.set_c(val & 1 != 0);

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_absolute_x<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	// Shift right
	let mut result = (val >> 1) as u8;
	state.cpu.p.set_c(val & 1 != 0);

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_absolute_y<M: Mapper, F>(mut state: State<M, F>, val: u16) -> State<M, F> {
	// Shift right
	let mut result = (val >> 1) as u8;
	state.cpu.p.set_c(val & 1 != 0);

	// XOR with accumulator
	result ^= state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(val & 0xFF, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_indirect_x<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	state.cpu.p.set_c(val & 1 != 0);

	// Shift right
	// XOR with accumulator
	let result = val >> 1 ^ state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn sre_indirect_y<M: Mapper, F>(mut state: State<M, F>, adr: u8) -> State<M, F> {
	let val = state.mem(adr as u16);

	state.cpu.p.set_c(val & 1 != 0);

	// Shift right
	let result = (val >> 1) ^ state.cpu.a;

	// Update flags
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	state.set_mem(adr as u16, result);
	state.cpu.a = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn anc_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND with accumulator
	state.cpu.a &= val;

	// Update flags
	state.cpu.p.set_z(state.cpu.a == 0);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);

	// Set carry flag to bit 7 of accumulator
	state.cpu.p.set_c((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn alr_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND with accumulator
	state.cpu.a &= val;

	// Shift right
	state.cpu.a >>= 1;
	state.cpu.p.set_c(state.cpu.a & 1 != 0);

	// Update flags
	state.cpu.p.set_z(state.cpu.a == 0);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn arr_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND with accumulator
	state.cpu.a &= val;

	// Shift right with carry
	state.cpu.a >>= 1;
	state.cpu.a |= (state.cpu.p.c() as u8) << 7;
	state.cpu.p.set_c(state.cpu.a & 1 != 0);

	// Update flags
	state.cpu.p.set_z(state.cpu.a == 0);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn axs_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND accumulator with X register
	let temp = state.cpu.a & state.cpu.x;

	// Subtract with borrow
	let result = temp - val - (1 - state.cpu.p.c() as u8);
	state.cpu.p.set_c(result <= temp);
	state.cpu.p.set_z(result == 0);
	state.cpu.p.set_n((result & 0x80) != 0);

	// Store result in X register
	state.cpu.x = result;
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn las_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND with accumulator and store in A, X, and S
	state.cpu.a &= val;
	state.cpu.x = state.cpu.a;
	state.cpu.s = state.cpu.a;

	// Update flags
	state.cpu.p.set_z(state.cpu.a == 0);
	state.cpu.p.set_n((state.cpu.a & 0x80) != 0);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn tas_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	// AND accumulator with X register and store in S
	let temp = state.cpu.a & state.cpu.x;
	state.cpu.s = temp;

	// Store S in memory
	state.set_mem(val as u16, temp);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn tas_absolute_y<M: Mapper, F>(mut state: State<M, F>, _: u16) -> State<M, F> {
	state.cpu.pc += 3;
	state
}

#[inline(always)]
pub fn shy_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	let adr = u16::from_le_bytes([val, val.wrapping_add(1)]);
	state.set_mem(adr, state.cpu.y);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn shy_absolute_x<M: Mapper, F>(mut state: State<M, F>, _: u16) -> State<M, F> {
	state.cpu.pc += 3;
	state
}

#[inline(always)]
pub fn shx_immediate<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	let adr = u16::from_le_bytes([val, val.wrapping_add(1)]);
	state.set_mem(adr, state.cpu.x);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn shx_absolute_y<M: Mapper, F>(mut state: State<M, F>, _: u16) -> State<M, F> {
	state.cpu.pc += 3;
	state
}

#[inline(always)]
pub fn ahx_absolute_y<M: Mapper, F>(mut state: State<M, F>, adr: u16) -> State<M, F> {
	state.set_mem(adr, state.cpu.a & state.cpu.x);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn ahx_indirect_y<M: Mapper, F>(mut state: State<M, F>, val: u8) -> State<M, F> {
	let adr = u16::from_le_bytes([val, val.wrapping_add(1)]);
	state.set_mem(adr, state.cpu.a & state.cpu.x);
	state.cpu.pc += 2;
	state
}

#[inline(always)]
pub fn stp<M: Mapper, F>(mut state: State<M, F>) -> State<M, F> {
	state.cpu.pc += 1;
	state.rest.ppu_runahead += (1);
	state
}

#[inline(always)]
pub fn xaa_immediate<M: Mapper, F>(mut state: State<M, F>, _: u8) -> State<M, F> {
	state.cpu.pc += 2;
	state.rest.ppu_runahead += (1);
	state
}

#[inline(always)]
pub fn las_absolute_y<M: Mapper, F>(mut state: State<M, F>, _: u16) -> State<M, F> {
	state.cpu.pc += 2;
	state.rest.ppu_runahead += (1);
	state
}
