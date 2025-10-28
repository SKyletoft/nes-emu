use std::ops::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct U15(u16);

impl U15 {
	const MASK: u16 = 0x7FFF;

	pub fn new(val: u16) -> Self {
		val.into()
	}

	pub fn get(self) -> u16 {
		self.into()
	}
}

impl From<u16> for U15 {
	fn from(val: u16) -> Self {
		U15(val)
	}
}

impl From<U15> for u16 {
	fn from(val: U15) -> u16 {
		val.0 & U15::MASK
	}
}

impl PartialEq for U15 {
	fn eq(&self, other: &Self) -> bool {
		u16::from(*self) == u16::from(*other)
	}
}

impl Eq for U15 {}

impl PartialOrd for U15 {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for U15 {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		u16::from(*self).cmp(&u16::from(*other))
	}
}

impl Add for U15 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		U15(self.0 + rhs.0)
	}
}

impl Sub for U15 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		U15(self.0.wrapping_sub(rhs.0))
	}
}

impl Mul for U15 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		U15(self.0 * rhs.0)
	}
}

impl Div for U15 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		U15(self.0 / rhs.0)
	}
}

impl Rem for U15 {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output {
		U15(self.0 % rhs.0)
	}
}

impl BitAnd for U15 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		U15(self.0 & rhs.0)
	}
}

impl BitOr for U15 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		U15(self.0 | rhs.0)
	}
}

impl BitXor for U15 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		U15(self.0 ^ rhs.0)
	}
}

impl Not for U15 {
	type Output = Self;
	fn not(self) -> Self::Output {
		U15(!self.0)
	}
}

impl Shl<u32> for U15 {
	type Output = Self;
	fn shl(self, rhs: u32) -> Self::Output {
		U15(self.0 << rhs)
	}
}

impl Shr<u32> for U15 {
	type Output = Self;
	fn shr(self, rhs: u32) -> Self::Output {
		U15(self.0 >> rhs)
	}
}

impl std::fmt::Display for U15 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", u16::from(*self))
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, Default)]
pub struct U3(u8);

impl U3 {
	const MASK: u8 = 0x07;

	pub fn new(val: u8) -> Self {
		val.into()
	}

	pub fn get(self) -> u8 {
		self.into()
	}
}

impl From<u8> for U3 {
	fn from(val: u8) -> Self {
		U3(val)
	}
}

impl From<U3> for u8 {
	fn from(val: U3) -> u8 {
		val.0 & U3::MASK
	}
}

impl PartialEq for U3 {
	fn eq(&self, other: &Self) -> bool {
		u8::from(*self) == u8::from(*other)
	}
}

impl Eq for U3 {}

impl PartialOrd for U3 {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for U3 {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		u8::from(*self).cmp(&u8::from(*other))
	}
}

impl Add for U3 {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		U3(self.0.wrapping_add(rhs.0))
	}
}

impl Sub for U3 {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		U3(self.0.wrapping_sub(rhs.0))
	}
}

impl Mul for U3 {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		U3(self.0.wrapping_mul(rhs.0))
	}
}

impl Div for U3 {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		U3(self.0 / rhs.0)
	}
}

impl Rem for U3 {
	type Output = Self;
	fn rem(self, rhs: Self) -> Self::Output {
		U3(self.0 % rhs.0)
	}
}

impl BitAnd for U3 {
	type Output = Self;
	fn bitand(self, rhs: Self) -> Self::Output {
		U3(self.0 & rhs.0)
	}
}

impl BitOr for U3 {
	type Output = Self;
	fn bitor(self, rhs: Self) -> Self::Output {
		U3(self.0 | rhs.0)
	}
}

impl BitXor for U3 {
	type Output = Self;
	fn bitxor(self, rhs: Self) -> Self::Output {
		U3(self.0 ^ rhs.0)
	}
}

impl Not for U3 {
	type Output = Self;
	fn not(self) -> Self::Output {
		U3(!self.0)
	}
}

impl Shl<u32> for U3 {
	type Output = Self;
	fn shl(self, rhs: u32) -> Self::Output {
		U3(self.0 << rhs)
	}
}

impl Shr<u32> for U3 {
	type Output = Self;
	fn shr(self, rhs: u32) -> Self::Output {
		U3(self.0 >> rhs)
	}
}

impl std::fmt::Display for U3 {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", u8::from(*self))
	}
}
