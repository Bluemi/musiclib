use std::collections::HashSet;

use crate::keys::Key;
use crate::intervals::Interval;
use crate::pitch::{Pitch, Octave, PitchRange};

/*
 * Represents a set of Intervals, which relative to a keynote build a chord.
 */
pub struct ChordTemplate {
	pub intervals: HashSet<Interval>,
}

pub struct Chord {
	pub keys: HashSet<Key>,
}

pub struct PitchChord {
	pub pitches: HashSet<Pitch>,
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

impl PitchChord {
	pub fn from_chord_and_octave(chord: &Chord, octave: Octave) -> PitchChord {
		let pitches = chord.keys.iter().map(|key| Pitch::from_key_and_octave(*key, octave)).collect();
		PitchChord { pitches }
	}

	pub fn from_chord_and_pitch_range(chord: Chord, pitch_range: PitchRange) -> PitchChord {
		let mut pitches = Vec::new();
		for key in chord.keys {
			let mut vec = pitch_range.inner_pitches(key);
			pitches.append(&mut vec);
		}
		PitchChord { pitches: pitches.iter().cloned().collect() }
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

	#[test]
	pub fn pitch_chord_from_chord_and_pitch_range() {
		let pitch_range = PitchRange { lower: Pitch { value: 24 }, upper: Pitch { value: 42 }};
		let chord = Chord::from_chord_template(&ChordTemplate::major(), Key::A);
		let pitch_chord = PitchChord::from_chord_and_pitch_range(chord, pitch_range);
		let mut asserted_pitches = HashSet::new();
		asserted_pitches.insert(Pitch { value: 24 });
		asserted_pitches.insert(Pitch { value: 28 });
		asserted_pitches.insert(Pitch { value: 31 });
		asserted_pitches.insert(Pitch { value: 36 });
		asserted_pitches.insert(Pitch { value: 40 });

		assert_eq!(pitch_chord.pitches, asserted_pitches);
		assert_eq!(pitch_chord.pitches.len(), 5);
	}
}
