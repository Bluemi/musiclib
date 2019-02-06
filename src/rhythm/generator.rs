use super::{Duration, TimePoint, RhythmNote};

struct StraitRhythmNotes {
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
		Some(RhythmNote { time_point: cur, duration: self.duration })
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_straight_rhythm_notes() {
		let duration = Duration::new(1, 4);
		let srn: StraitRhythmNotes = StraitRhythmNotes::with_duration(duration);

		let mut rn = RhythmNote::new(TimePoint::new(0, 1), duration);
		for note in srn.take(20) {
			assert_eq!(note, rn);
			rn.time_point += duration;
		}
	}
}
