use super::{Duration, TimePoint, RhythmNote, BarTimeSignature};

pub struct StraitRhythmNotes {
	time_point: TimePoint,
	duration: Duration,
}

impl StraitRhythmNotes {
	pub fn with_duration(duration: Duration) -> StraitRhythmNotes {
		StraitRhythmNotes { time_point: TimePoint::new(0, 1), duration: duration }
	}
}

impl Iterator for StraitRhythmNotes {
	type Item = RhythmNote;

	fn next(&mut self) -> Option<Self::Item> {
		let cur = self.time_point;
		self.time_point += self.duration;
		Some(RhythmNote::new(cur, self.duration))
	}
}

pub struct RhythmPattern {
	pub notes: Vec<RhythmNote>,
}

impl RhythmPattern {
	pub fn straight_rhythm_notes(bar_time_signature: BarTimeSignature, duration: Duration) -> RhythmPattern {
		let mut counter = TimePoint::new(0, 1);

		let mut notes: Vec<RhythmNote> = vec![RhythmNote::new(counter, duration)];

		let end: TimePoint = TimePoint::from(bar_time_signature) - duration;

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
	pub fn test_straight_rhythm_notes_pattern() {
		let rhythm_pattern1: RhythmPattern = RhythmPattern::straight_rhythm_notes(BarTimeSignature::four_quarter_time(), Duration::quarter());
		let asserted_pattern1: Vec<RhythmNote> = vec![
			RhythmNote::new(TimePoint::new(0, 4), Duration::quarter()),
			RhythmNote::new(TimePoint::new(1, 4), Duration::quarter()),
			RhythmNote::new(TimePoint::new(2, 4), Duration::quarter()),
			RhythmNote::new(TimePoint::new(3, 4), Duration::quarter()),
		];
		assert_eq!(rhythm_pattern1.notes, asserted_pattern1);

		let rhythm_pattern2: RhythmPattern = RhythmPattern::straight_rhythm_notes(BarTimeSignature::new(6, 8), Duration::new(3, 8));
		let asserted_pattern2: Vec<RhythmNote> = vec![
			RhythmNote::new(TimePoint::new(0, 4), Duration::new(3, 8)),
			RhythmNote::new(TimePoint::new(3, 8), Duration::new(3, 8)),
		];
		assert_eq!(rhythm_pattern2.notes, asserted_pattern2);
	}


	#[test]
	pub fn test_straight_rhythm_notes_gen() {
		let duration = Duration::new(1, 4);
		let srn: StraitRhythmNotes = StraitRhythmNotes::with_duration(duration);

		let mut rn = RhythmNote::new(TimePoint::new(0, 1), duration);
		for note in srn.take(20) {
			assert_eq!(note, rn);
			rn.time_point += duration;
		}
	}
}
