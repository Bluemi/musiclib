use std::ops::Add;
use num_rational::Rational32;
use num_traits::identities::Zero;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Duration {
	pub duration: Rational32,
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
}

/*
 * A 4/4 Bar has the BarTimePoints: [0/4, 1/4, 2/4, 3/4]
 */
#[derive(Clone, Copy, PartialEq)]
pub struct BarTimePoint {
	pub time_point: Rational32,
}

impl Add<Duration> for BarTimePoint {
	type Output = BarTimePoint;

	fn add(self, duration: Duration) -> BarTimePoint {
		BarTimePoint { time_point: self.time_point + duration.duration }
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct BarTimeSignature {
	pub nominator: u32,
	pub denominator: u32,
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

	pub fn get_ratio(self) -> Rational32 {
		Rational32::new(self.nominator as i32, self.denominator as i32)
	}
}

#[derive(Clone, Copy, PartialEq)]
pub struct RhythmNote {
	pub time_point: BarTimePoint,
	pub duration: Duration,
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

impl RhythmNote {
	pub fn new(time_point: Rational32, duration: Rational32) -> RhythmNote {
		RhythmNote {
			time_point: BarTimePoint { time_point },
			duration: Duration { duration }
		}
	}
}

pub struct RhythmPattern {
	pub notes: Vec<RhythmNote>,
}

impl RhythmPattern {
	pub fn straight_rhythm_notes(bar_time_signature: BarTimeSignature, duration: Rational32) -> RhythmPattern {
		let mut counter = Rational32::zero();

		let mut notes: Vec<RhythmNote> = vec![RhythmNote::new(counter, duration)];

		while counter < (bar_time_signature.get_ratio() - duration) {
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
		let rhythm_pattern: RhythmPattern = RhythmPattern::straight_rhythm_notes(BarTimeSignature::four_quarter_time(), Duration::quarter().duration);
		let asserted_pattern: Vec<RhythmNote> = vec![
			RhythmNote::new(Rational32::new(0, 4), Rational32::new(1, 4)),
			RhythmNote::new(Rational32::new(1, 4), Rational32::new(1, 4)),
			RhythmNote::new(Rational32::new(2, 4), Rational32::new(1, 4)),
			RhythmNote::new(Rational32::new(3, 4), Rational32::new(1, 4)),
		];
		assert_eq!(rhythm_pattern.notes, asserted_pattern);
	}
}
