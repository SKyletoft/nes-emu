use bitfields::bitfield;

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq)]
pub struct ControllerState {
	a: bool,
	b: bool,
	select: bool,
	start: bool,
	up: bool,
	down: bool,
	left: bool,
	right: bool,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Default)]
pub enum LatchState {
	#[default]
	A,
	B,
	Select,
	Start,
	Up,
	Down,
	Left,
	Right,
	One,
}

#[derive(Copy, Clone, PartialEq, Default)]
pub struct Controller {
	latch_state: LatchState,
	strobe: bool,
	controller_state: ControllerState,
}

impl Controller {
	pub fn state_mut(&mut self) -> &mut u8 {
		unsafe { std::mem::transmute::<&mut ControllerState, &mut u8>(&mut self.controller_state) }
	}

	pub fn write(&mut self, val: u8) {
		self.strobe = val == 1;
		if self.strobe {
			self.latch_state = LatchState::A;
		}
	}

	pub fn read_pure(&self) -> u8 {
		(match self.latch_state {
			LatchState::A => self.controller_state.a(),
			LatchState::B => self.controller_state.b(),
			LatchState::Select => self.controller_state.select(),
			LatchState::Start => self.controller_state.start(),
			LatchState::Up => self.controller_state.up(),
			LatchState::Down => self.controller_state.down(),
			LatchState::Left => self.controller_state.left(),
			LatchState::Right => self.controller_state.right(),
			LatchState::One => true,
		}) as u8
	}

	pub fn read(&mut self) -> u8 {
		let val = self.read_pure();
		if !self.strobe {
			match self.latch_state {
				LatchState::A => {
					self.latch_state = LatchState::B;
				}
				LatchState::B => {
					self.latch_state = LatchState::Select;
				}
				LatchState::Select => {
					self.latch_state = LatchState::Start;
				}
				LatchState::Start => {
					self.latch_state = LatchState::Up;
				}
				LatchState::Up => {
					self.latch_state = LatchState::Down;
				}
				LatchState::Down => {
					self.latch_state = LatchState::Left;
				}
				LatchState::Left => {
					self.latch_state = LatchState::Right;
				}
				LatchState::Right => {
					self.latch_state = LatchState::One;
				}
				LatchState::One => {}
			}
		}
		val
	}
}
