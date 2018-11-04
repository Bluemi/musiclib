#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interval {
	Prime,
	MinorSecond,
	MajorSecond,
	MinorThird,
	MajorThird,
	Fourth,
	Tritone,
	Fifth,
	MinorSixth,
	MajorSixth,
	MinorSeventh,
	MajorSeventh,
	Octave,
}

impl Interval {
	pub fn halftones(self) -> u32 {
		match self {
			Interval::Prime => 0,
			Interval::MinorSecond => 1,
			Interval::MajorSecond => 2,
			Interval::MinorThird => 3,
			Interval::MajorThird => 4,
			Interval::Fourth => 5,
			Interval::Tritone => 6,
			Interval::Fifth => 7,
			Interval::MinorSixth => 8,
			Interval::MajorSixth => 9,
			Interval::MinorSeventh => 10,
			Interval::MajorSeventh => 11,
			Interval::Octave => 12,
		}
	}
}
