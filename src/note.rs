use crate::rhythm::{TimePoint, Duration, RhythmNote};
use crate::pitch::Pitch;

#[derive(Clone, Copy, PartialEq)]
pub struct Note {
	pub time_point: TimePoint,
	pub duration: Duration,
	pub pitch: Pitch,
}

impl Note {
    pub fn get_rhythm_note(&self) -> RhythmNote {
        RhythmNote::new(self.time_point, self.duration)
    }
}
