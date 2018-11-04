use std::ops::{Add, Sub};
use std::convert::From;
use std::fmt;

use crate::intervals::Interval;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Key { A, Ais, B, C, Cis, D, Dis, E, F, Fis, G, Gis, }

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

		((number+12) - halftones).into()
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
		match number%12 {
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
			_ => panic!("number % 12 is >= 12"),
		}
	}
}

impl fmt::Display for Key {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Key::A => write!(f, "A Key"),
			Key::Ais => write!(f, "Ais Key"),
			Key::B => write!(f, "B Key"),
			Key::C => write!(f, "C Key"),
			Key::Cis => write!(f, "Cis Key"),
			Key::D => write!(f, "D Key"),
			Key::Dis => write!(f, "Dis Key"),
			Key::E => write!(f, "E Key"),
			Key::F => write!(f, "F Key"),
			Key::Fis => write!(f, "Fis Key"),
			Key::G => write!(f, "G Key"),
			Key::Gis => write!(f, "Gis Key"),
		}
	}
}
