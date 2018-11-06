use std::convert::From;

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

impl From<u32> for Interval {
	fn from(value: u32) -> Interval {
		match value {
			0 => Interval::Prime,
			1 => Interval::MinorSecond,
			2 => Interval::MajorSecond,
			3 => Interval::MinorThird,
			4 => Interval::MajorThird,
			5 => Interval::Fourth,
			6 => Interval::Tritone,
			7 => Interval::Fifth,
			8 => Interval::MinorSixth,
			9 => Interval::MajorSixth,
			10 => Interval::MinorSeventh,
			11 => Interval::MajorSeventh,
			12 => Interval::Octave,
			_ => panic!("can not convert {} into an Interval", value),
		}
	}
}
