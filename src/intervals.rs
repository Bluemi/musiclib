#[derive(Clone, Copy)]
pub enum Interval {
	Prime,
	LSecond,
	BSecond,
	LThird,
	BThird,
	Fourth,
	Tritone,
	Fifth,
	LSixth,
	BSixth,
	LSeventh,
	BSeventh,
	Octave,
}

impl Interval {
	pub fn halftones(self) -> u32 {
		match self {
			Interval::Prime => 0,
			Interval::LSecond => 1,
			Interval::BSecond => 2,
			Interval::LThird => 3,
			Interval::BThird => 4,
			Interval::Fourth => 5,
			Interval::Tritone => 6,
			Interval::Fifth => 7,
			Interval::LSixth => 8,
			Interval::BSixth => 9,
			Interval::LSeventh => 10,
			Interval::BSeventh => 11,
			Interval::Octave => 12,
		}
	}
}
