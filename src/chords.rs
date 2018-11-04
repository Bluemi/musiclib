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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn major_chords() {
		let major_chord_template = ChordTemplate::major();

		let major_a_chord = Chord::from_chord_template(&major_chord_template, Key::A);
		assert!(major_a_chord.keys.contains(&Key::A));
		assert!(major_a_chord.keys.contains(&Key::Cis));
		assert!(major_a_chord.keys.contains(&Key::E));
		assert_eq!(major_a_chord.keys.len(), 3);

		let major_cis_chord = Chord::from_chord_template(&major_chord_template, Key::Cis);
		assert!(major_cis_chord.keys.contains(&Key::Cis));
		assert!(major_cis_chord.keys.contains(&Key::F));
		assert!(major_cis_chord.keys.contains(&Key::Gis));
		assert_eq!(major_cis_chord.keys.len(), 3);
	}

	#[test]
	pub fn minor_chords() {
		let minor_chord_template = ChordTemplate::minor();

		let minor_a_chord = Chord::from_chord_template(&minor_chord_template, Key::Gis);
		assert!(minor_a_chord.keys.contains(&Key::Gis));
		assert!(minor_a_chord.keys.contains(&Key::B));
		assert!(minor_a_chord.keys.contains(&Key::Dis));
		assert_eq!(minor_a_chord.keys.len(), 3);

		let minor_cis_chord = Chord::from_chord_template(&minor_chord_template, Key::Cis);
		assert!(minor_cis_chord.keys.contains(&Key::Cis));
		assert!(minor_cis_chord.keys.contains(&Key::E));
		assert!(minor_cis_chord.keys.contains(&Key::Gis));
		assert_eq!(minor_cis_chord.keys.len(), 3);
	}
}
