use std::ops::{Add, Sub};
use std::convert::From;
use std::fmt;

use crate::intervals::Interval;
use crate::pitch::Pitch;
use crate::math::neg_modulo;

const NUM_HALF_TONES: u32 = 12;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Key { A, Ais, B, C, Cis, D, Dis, E, F, Fis, G, Gis, }

impl Key {
	pub fn to_interval(lower_key: Key, upper_key: Key) -> Interval {
		let lower: u32 = lower_key.into();
		let upper: u32 = upper_key.into();

		((upper+12-lower)%12).into()
	}
}

impl Add<Interval> for Key {
	type Output = Key;

	fn add(self, interval: Interval) -> Key {
		let mut number: u32 = self.into();

		number += interval.halftones();
		number.into()
	}
}

impl Sub<Interval> for Key {
	type Output = Key;

	fn sub(self, interval: Interval) -> Key {
		let number: u32 = self.into();
		let halftones = interval.halftones();

		((number+NUM_HALF_TONES) - halftones).into()
	}
}

impl From<Pitch> for Key {
	fn from(pitch: Pitch) -> Key {
		neg_modulo(pitch.value, NUM_HALF_TONES).into()
	}
}

impl From<Key> for u32 {
	fn from(key: Key) -> u32 {
		match key {
			Key::A => 0,
			Key::Ais => 1,
			Key::B => 2,
			Key::C => 3,
			Key::Cis => 4,
			Key::D => 5,
			Key::Dis => 6,
			Key::E => 7,
			Key::F => 8,
			Key::Fis => 9,
			Key::G => 10,
			Key::Gis => 11,
		}
	}
}

impl From<u32> for Key {
	fn from(number: u32) -> Key {
		match number%NUM_HALF_TONES {
			0 => Key::A,
			1 => Key::Ais,
			2 => Key::B,
			3 => Key::C,
			4 => Key::Cis,
			5 => Key::D,
			6 => Key::Dis,
			7 => Key::E,
			8 => Key::F,
			9 => Key::Fis,
			10 => Key::G,
			11 => Key::Gis,
			_ => panic!("number % {0} is >= {0}", NUM_HALF_TONES),
		}
	}
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Key::A => write!(f, "Key A"),
			Key::Ais => write!(f, "Key Ais"),
			Key::B => write!(f, "Key B"),
			Key::C => write!(f, "Key C"),
			Key::Cis => write!(f, "Key Cis"),
			Key::D => write!(f, "Key D"),
			Key::Dis => write!(f, "Key Dis"),
			Key::E => write!(f, "Key E"),
			Key::F => write!(f, "Key F"),
			Key::Fis => write!(f, "Key Fis"),
			Key::G => write!(f, "Key G"),
			Key::Gis => write!(f, "Key Gis"),
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn key_add_interval() {
		assert_eq!(Key::B, Key::A + Interval::MajorSecond);
		assert_eq!(Key::C, Key::A + Interval::Octave + Interval::MinorThird);
		assert_eq!(Key::F, Key::Cis + Interval::MajorThird);
	}

	#[test]
	pub fn key_sub_interval() {
		assert_eq!(Key::G, Key::A - Interval::MajorSecond);
		assert_eq!(Key::Fis, Key::A - Interval::Octave - Interval::MinorThird);
	}

	#[test]
	pub fn from_pitch_to_key() {
		assert_eq!(Key::F, Key::from(Pitch { value: -4 }));
		assert_eq!(Key::B, Key::from(Pitch { value: 2 }));
	}
}
