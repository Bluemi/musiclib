#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

	pub fn invert(&mut self) {
		self.halftones = -self.halftones;
	}
}
