use std::collections::HashSet;

use crate::keys::Key;
use crate::intervals::Interval;

/*
 * Represents a set of Intervals, which relative to a keynote build a chord.
 */
pub struct ChordTemplate {
	pub intervals: HashSet<Interval>,
}

pub struct Chord {
	pub keys: HashSet<Key>,
}

impl ChordTemplate {
	pub fn new() -> ChordTemplate {
		ChordTemplate { intervals: HashSet::new() }
	}

	pub fn major() -> ChordTemplate {
		let mut chord_template: ChordTemplate = ChordTemplate::new();
		chord_template.intervals.insert(Interval::Prime);
		chord_template.intervals.insert(Interval::MajorThird);
		chord_template.intervals.insert(Interval::Fifth);
		chord_template
	}

	pub fn minor() -> ChordTemplate {
		let mut chord_template: ChordTemplate = ChordTemplate::new();
		chord_template.intervals.insert(Interval::Prime);
		chord_template.intervals.insert(Interval::MinorThird);
		chord_template.intervals.insert(Interval::Fifth);
		chord_template
	}
}

impl Chord {
	pub fn from_chord_template(chord_template: &ChordTemplate, keynote: Key) -> Chord {
		let keys = chord_template.intervals.iter().map(|interval| keynote + *interval).collect();
		Chord { keys }
	}
}
