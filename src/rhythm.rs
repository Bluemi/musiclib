use std::ops::{Add,Sub,AddAssign};
use num_rational::Rational32;
use std::fmt;
use std::convert::From;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Duration {
	duration: Rational32,
}

impl Duration {
	pub fn whole() -> Duration {
		Duration { duration: Rational32::new(1, 1) }
	}

	pub fn half() -> Duration {
		Duration { duration: Rational32::new(1, 2) }
	}

	pub fn quarter() -> Duration {
		Duration { duration: Rational32::new(1, 4) }
	}

	pub fn eighth() -> Duration {
		Duration { duration: Rational32::new(1, 8) }
	}

	pub fn sixteenth() -> Duration {
		Duration { duration: Rational32::new(1, 16) }
	}

	pub fn from_rational(rational: Rational32) -> Duration {
		Duration { duration: rational }
	}
}

impl From<Duration> for Rational32 {
	fn from(duration: Duration) -> Rational32 {
		duration.duration
	}
}

impl From<BarTimePoint> for Duration {
	fn from(time_point: BarTimePoint) -> Duration {
		Duration { duration: time_point.time_point }
	}
}

/*
 * A 4/4 Bar has the BarTimePoints: [0/4, 1/4, 2/4, 3/4]
 */
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct BarTimePoint {
	time_point: Rational32,
}

impl BarTimePoint {
	pub fn new(nominator: u32, denominator: u32) -> BarTimePoint {
		BarTimePoint { time_point: Rational32::new(nominator as i32, denominator as i32) }
	}
}

impl Add<Duration> for BarTimePoint {
	type Output = BarTimePoint;

	fn add(self, duration: Duration) -> BarTimePoint {
		BarTimePoint { time_point: self.time_point + duration.duration }
	}
}

impl AddAssign<Duration> for BarTimePoint {
	fn add_assign(&mut self, duration: Duration) {
		self.time_point += duration.duration;
	}
}

impl Sub<Duration> for BarTimePoint {
	type Output = BarTimePoint;

	fn sub(self, duration: Duration) -> BarTimePoint {
		BarTimePoint { time_point: self.time_point - duration.duration }
	}
}

impl From<BarTimeSignature> for BarTimePoint {
	fn from(bar_time_signature: BarTimeSignature) -> BarTimePoint {
		BarTimePoint { time_point: Rational32::new(bar_time_signature.nominator as i32, bar_time_signature.denominator as i32) }
	}
}

impl From<BarTimePoint> for Rational32 {
	fn from(time_point: BarTimePoint) -> Rational32 {
		time_point.time_point
	}
}

impl From<Rational32> for BarTimePoint {
	fn from(rational: Rational32) -> BarTimePoint {
		 BarTimePoint { time_point: rational }
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct BarTimeSignature {
	nominator: u32,
	denominator: u32,
}

impl BarTimeSignature {
	pub fn four_quarter_time() -> BarTimeSignature {
		BarTimeSignature { nominator: 4, denominator: 4 }
	}

	pub fn three_quarter_time() -> BarTimeSignature {
		BarTimeSignature { nominator: 3, denominator: 4 }
	}

	pub fn two_quarter_time() -> BarTimeSignature {
		BarTimeSignature { nominator: 2, denominator: 4 }
	}

	pub fn six_eighth_time() -> BarTimeSignature {
		BarTimeSignature { nominator: 6, denominator: 8 }
	}

	pub fn get_nominator(self) -> u32 {
		self.nominator
	}

	pub fn get_denominator(self) -> u32 {
		self.denominator
	}

	pub fn new(nominator: u32, denominator: u32) -> BarTimeSignature {
		BarTimeSignature { nominator, denominator }
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct RhythmNote {
	time_point: BarTimePoint,
	duration: Duration,
}

impl RhythmNote {
	pub fn new(time_point: BarTimePoint, duration: Duration) -> RhythmNote {
		RhythmNote {
			time_point,
			duration
		}
	}
}

impl fmt::Debug for RhythmNote {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "RhythmNote: time_point: {}/{}, duration: {}/{}",
			   self.time_point.time_point.numer(),
			   self.time_point.time_point.denom(),
			   self.duration.duration.numer(),
			   self.duration.duration.denom())
	}
}

pub struct RhythmPattern {
	pub notes: Vec<RhythmNote>,
}

impl RhythmPattern {
	pub fn straight_rhythm_notes(bar_time_signature: BarTimeSignature, duration: Duration) -> RhythmPattern {
		let mut counter = BarTimePoint::new(0, 1);

		let mut notes: Vec<RhythmNote> = vec![RhythmNote::new(counter, duration)];

		let end: BarTimePoint = BarTimePoint::from(bar_time_signature) - duration;

		while counter < end {
			counter += duration;
			notes.push(RhythmNote::new(counter, duration))
		}

		RhythmPattern { notes }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_straight_rhythm_notes() {
		let rhythm_pattern1: RhythmPattern = RhythmPattern::straight_rhythm_notes(BarTimeSignature::four_quarter_time(), Duration::quarter());
		let asserted_pattern1: Vec<RhythmNote> = vec![
			RhythmNote::new(BarTimePoint::new(0, 4), Duration::quarter()),
			RhythmNote::new(BarTimePoint::new(1, 4), Duration::quarter()),
			RhythmNote::new(BarTimePoint::new(2, 4), Duration::quarter()),
			RhythmNote::new(BarTimePoint::new(3, 4), Duration::quarter()),
		];
		assert_eq!(rhythm_pattern1.notes, asserted_pattern1);
	}
}
