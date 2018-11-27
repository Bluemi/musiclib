use crate::rhythm::{BarTimePoint, Duration};
use crate::pitch::Pitch;

pub struct Note {
	pub time_point: BarTimePoint,
	pub duration: Duration,
	pub pitch: Pitch;
}

pub struct Bar {
	pub notes: Vec<Note>,
}
