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
}
