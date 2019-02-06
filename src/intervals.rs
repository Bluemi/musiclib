use std::fmt;

use crate::math::neg_modulo;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Interval {
	pub halftones: i32,
}

impl Interval {
	pub fn prime() -> Interval { Interval { halftones: 0 } }
	pub fn minor_second() -> Interval { Interval { halftones: 1 } }
	pub fn major_second() -> Interval { Interval { halftones: 2 } }
	pub fn minor_third() -> Interval { Interval { halftones: 3 } }
	pub fn major_third() -> Interval { Interval { halftones: 4 } }
	pub fn fourth() -> Interval { Interval { halftones: 5 } }
	pub fn tritone() -> Interval { Interval { halftones: 6 } }
	pub fn fifth() -> Interval { Interval { halftones: 7 } }
	pub fn minor_sixth() -> Interval { Interval { halftones: 8 } }
	pub fn major_sixth() -> Interval { Interval { halftones: 9 } }
	pub fn minor_seventh() -> Interval { Interval { halftones: 10 } }
	pub fn major_seventh() -> Interval { Interval { halftones: 11 } }
	pub fn octave() -> Interval { Interval { halftones: 12 } }
	pub fn minor_ninth() -> Interval { Interval { halftones: 13 } }
	pub fn major_ninth() -> Interval { Interval { halftones: 14 } }
	pub fn minor_tenth() -> Interval { Interval { halftones: 15 } }
	pub fn major_tenth() -> Interval { Interval { halftones: 16 } }
	pub fn eleventh() -> Interval { Interval { halftones: 17 } }
	pub fn diminished_twelfth() -> Interval { Interval { halftones: 18 } }
	pub fn augmented_eleventh() -> Interval { Interval { halftones: 18 } }
	pub fn twelfth() -> Interval { Interval { halftones: 19 } }
	pub fn minor_thirteenth() -> Interval { Interval { halftones: 20 } }
	pub fn major_thirteenth() -> Interval { Interval { halftones: 21 } }
	pub fn minor_fourteenth() -> Interval { Interval { halftones: 22 } }
	pub fn major_fourteenth() -> Interval { Interval { halftones: 23 } }
	pub fn fifteenth() -> Interval { Interval { halftones: 24 } }

	pub fn invert(&mut self) {
		self.halftones = -self.halftones;
	}

	pub fn to_base_interval(self) -> Interval {
		Interval { halftones: neg_modulo(self.halftones, Interval::octave().halftones as u32) as i32 }
	}

	pub fn get_num_octaves(self) -> i32 {
		self.halftones / Interval::octave().halftones
	}
}

impl fmt::Display for Interval {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.halftones % Interval::octave().halftones {
			0 => write!(f, "prime"),
			1 => write!(f, "minor_second"),
			2 => write!(f, "major_second"),
			3 => write!(f, "minor_third"),
			4 => write!(f, "major_third"),
			5 => write!(f, "fourth"),
			6 => write!(f, "tritone"),
			7 => write!(f, "fifth"),
			8 => write!(f, "minor_sixth"),
			9 => write!(f, "major_sixth"),
			10 => write!(f, "minor_seventh"),
			11 => write!(f, "major_seventh"),
			_ => unimplemented!(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UnspecifiedInterval {
	pub value: i32,
}

impl UnspecifiedInterval {
	pub fn new(value: i32) -> UnspecifiedInterval { UnspecifiedInterval { value } }

	pub fn prime() -> UnspecifiedInterval { UnspecifiedInterval { value: 0 } }
	pub fn second() -> UnspecifiedInterval { UnspecifiedInterval { value: 1 } }
	pub fn third() -> UnspecifiedInterval { UnspecifiedInterval { value: 2 } }
	pub fn fourth() -> UnspecifiedInterval { UnspecifiedInterval { value: 3 } }
	pub fn fifth() -> UnspecifiedInterval { UnspecifiedInterval { value: 4 } }
	pub fn sixth() -> UnspecifiedInterval { UnspecifiedInterval { value: 5 } }
	pub fn seventh() -> UnspecifiedInterval { UnspecifiedInterval { value: 6 } }
	pub fn octave() -> UnspecifiedInterval { UnspecifiedInterval { value: 7 } }
	pub fn ninth() -> UnspecifiedInterval { UnspecifiedInterval { value: 8 } }
	pub fn tenth() -> UnspecifiedInterval { UnspecifiedInterval { value: 9 } }
	pub fn eleventh() -> UnspecifiedInterval { UnspecifiedInterval { value: 10 } }
	pub fn twelfth() -> UnspecifiedInterval { UnspecifiedInterval { value: 11 } }
	pub fn thirteenth() -> UnspecifiedInterval { UnspecifiedInterval { value: 12 } }
	pub fn fourteenth() -> UnspecifiedInterval { UnspecifiedInterval { value: 13 } }
	pub fn fifteenth() -> UnspecifiedInterval { UnspecifiedInterval { value: 14 } }

	pub fn to_base_unspecified_interval(self) -> UnspecifiedInterval {
		UnspecifiedInterval {
			value: neg_modulo(self.value, UnspecifiedInterval::octave().value as u32 ) as i32
		}
	}

	pub fn get_num_octaves(self) -> i32 {
		self.value / UnspecifiedInterval::octave().value
	}

	pub fn to_minor_or_perfect(self) -> Interval {
		let mut interval: Interval = match self.to_base_unspecified_interval().value {
			0 => Interval::prime(),
			1 => Interval::minor_second(),
			2 => Interval::minor_third(),
			3 => Interval::fourth(),
			4 => Interval::fifth(),
			5 => Interval::minor_sixth(),
			6 => Interval::minor_seventh(),
			_ => unimplemented!(),
		};
		interval.halftones += Interval::octave().halftones * self.get_num_octaves();
		interval
	}

	pub fn to_major_or_perfect(self) -> Interval {
		let mut interval: Interval = match self.to_base_unspecified_interval().value {
			0 => Interval::prime(),
			1 => Interval::major_second(),
			2 => Interval::major_third(),
			3 => Interval::fourth(),
			4 => Interval::fifth(),
			5 => Interval::major_sixth(),
			6 => Interval::major_seventh(),
			_ => unimplemented!(),
		};
		interval.halftones += Interval::octave().halftones * self.get_num_octaves();
		interval
	}
}

impl fmt::Display for UnspecifiedInterval {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.value % UnspecifiedInterval::octave().value {
			0 => write!(f, "prime"),
			1 => write!(f, "second"),
			2 => write!(f, "third"),
			3 => write!(f, "fourth"),
			4 => write!(f, "fifth"),
			5 => write!(f, "sixth"),
			6 => write!(f, "seventh"),
			_ => unimplemented!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_to_major_or_perfect() {
		assert_eq!(UnspecifiedInterval::octave().to_major_or_perfect(), Interval::octave());
		assert_eq!(UnspecifiedInterval::third().to_major_or_perfect(), Interval::major_third());
		assert_eq!(UnspecifiedInterval::new(-1).to_major_or_perfect(), Interval::major_seventh());
		assert_eq!(UnspecifiedInterval::tenth().to_major_or_perfect(), Interval::major_tenth());
	}

	#[test]
	pub fn test_to_minor_or_perfect() {
		assert_eq!(UnspecifiedInterval::octave().to_minor_or_perfect(), Interval::octave());
		assert_eq!(UnspecifiedInterval::third().to_minor_or_perfect(), Interval::minor_third());
		assert_eq!(UnspecifiedInterval::new(-1).to_minor_or_perfect(), Interval::minor_seventh());
		assert_eq!(UnspecifiedInterval::tenth().to_minor_or_perfect(), Interval::minor_tenth());
	}
}
