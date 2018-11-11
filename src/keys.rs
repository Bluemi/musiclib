use std::ops::{Add, Sub};
use std::convert::From;
use std::fmt;

use crate::intervals::Interval;
use crate::pitch::Pitch;
use crate::math::neg_modulo;

const NUM_HALF_TONES: u8 = 12;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Key {
	value: u8,
}

impl Key {
	pub fn a() -> Key { Key { value : 0 } }
	pub fn ais() -> Key { Key { value : 1 } }
	pub fn b() -> Key { Key { value : 2 } }
	pub fn c() -> Key { Key { value : 3 } }
	pub fn cis() -> Key { Key { value : 4 } }
	pub fn d() -> Key { Key { value : 5 } }
	pub fn dis() -> Key { Key { value : 6 } }
	pub fn e() -> Key { Key { value : 7 } }
	pub fn f() -> Key { Key { value : 8 } }
	pub fn fis() -> Key { Key { value : 9 } }
	pub fn g() -> Key { Key { value : 10 } }
	pub fn gis() -> Key { Key { value : 11 } }

	pub fn to_interval(lower_key: Key, upper_key: Key) -> Interval {
		Interval { halftones: ((upper_key.value+NUM_HALF_TONES-lower_key.value)%NUM_HALF_TONES) as i32 }
	}

	pub fn new(value: u8) -> Key {
		Key { value }
	}

	pub fn get_value(self) -> u8 {
		self.value
	}
}

impl Add<Interval> for Key {
	type Output = Key;

	fn add(self, interval: Interval) -> Key {
		let number = neg_modulo(self.value as i32 + interval.halftones, NUM_HALF_TONES as u32) as u8;
		Key { value: number }
	}
}

impl Sub<Interval> for Key {
	type Output = Key;

	fn sub(self, mut interval: Interval) -> Key {
		interval.invert();
		self + interval
	}
}

impl From<Pitch> for Key {
	fn from(pitch: Pitch) -> Key {
		Key { value: neg_modulo(pitch.value, NUM_HALF_TONES as u32) as u8 }
	}
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.value % NUM_HALF_TONES {
			0 => write!(f, "a"),
			1 => write!(f, "a#"),
			2 => write!(f, "b"),
			3 => write!(f, "c"),
			4 => write!(f, "c#"),
			5 => write!(f, "d"),
			6 => write!(f, "d#"),
			7 => write!(f, "e"),
			8 => write!(f, "f"),
			9 => write!(f, "f#"),
			10 => write!(f, "g"),
			11 => write!(f, "g#"),
			_ => unimplemented!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn key_add_interval() {
		assert_eq!(Key::b(), Key::a() + Interval::major_second());
		assert_eq!(Key::c(), Key::a() + Interval::octave() + Interval::minor_third());
		assert_eq!(Key::f(), Key::cis() + Interval::major_third());
	}

	#[test]
	pub fn key_sub_interval() {
		assert_eq!(Key::g(), Key::a() - Interval::major_second());
		assert_eq!(Key::fis(), Key::a() - Interval::octave() - Interval::minor_third());
	}

	#[test]
	pub fn from_pitch_to_key() {
		assert_eq!(Key::f(), Key::from(Pitch { value: -4 }));
		assert_eq!(Key::b(), Key::from(Pitch { value: 2 }));
	}
}
