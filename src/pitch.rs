use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::fmt;

use crate::keys::Key;
use crate::intervals::Interval;

/*
 * the keyboard is divided into octaves starting with A.
 * value = 0, means a pitch of 27.5Hz or MIDI number 21
 * (see https://newt.phys.unsw.edu.au/jw/notes.html).
 * Negative values for Pitch.value are possible.
 */
#[derive(Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Pitch {
	pub value: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Octave {
	pub value: i32,
}

#[derive(Copy, Clone)]
pub struct PitchRange {
	pub lower: Pitch,
	pub upper: Pitch,
}

impl Pitch {
	pub fn from_key_and_octave(key: Key, octave: Octave) -> Pitch {
		Pitch { value: (octave.value * 12 + (key.get_value() as i32)) }
	}

	pub fn to_midi_number(&self) -> Option<u32> {
		if self.value >= -21 {
			Some((self.value + 21) as u32)
		} else {
			None
		}
	}

	pub fn get_upper_interval(self, upper_key: Key) -> Interval {
		let lower_key: Key = self.into();
		Key::to_interval(lower_key, upper_key)
	}

	pub fn get_overlying_pitch(self, key: Key) -> Pitch {
		let interval: Interval = Key::to_interval(self.into(), key);
		self + interval
	}

	pub fn get_underlying_pitch(self, key: Key) -> Pitch {
		let interval: Interval = Key::to_interval(key, self.into());
		self - interval
	}
}

impl Add<Interval> for Pitch {
	type Output = Pitch;

	fn add(self, interval: Interval) -> Pitch {
		Pitch { value: self.value + interval.halftones }
	}
}

impl Sub<Interval> for Pitch {
	type Output = Pitch;

	fn sub(self, interval: Interval) -> Pitch {
		Pitch { value: self.value - interval.halftones }
	}
}

impl AddAssign<Interval> for Pitch {
	fn add_assign(&mut self, interval: Interval) {
		self.value += interval.halftones;
	}
}

impl SubAssign<Interval> for Pitch {
	fn sub_assign(&mut self, interval: Interval) {
		self.value -= interval.halftones;
	}
}

impl fmt::Display for Pitch {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let key = Key::from(*self);
		key.fmt(f)
	}
}

impl PitchRange {
	pub fn inner_pitches(self, key: Key) -> Vec<Pitch> {
		let mut pitch: Pitch = self.lower.get_overlying_pitch(key);
		let mut pitches: Vec<Pitch> = Vec::new();

		while pitch < self.upper {
			pitches.push(pitch);
			pitch += Interval::octave();
		}

		pitches
	}
}
