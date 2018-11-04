#[allow(unused_imports)]
use crate::keys::Key;
#[allow(unused_imports)]
use crate::intervals::Interval;
#[allow(unused_imports)]
use crate::chords::{Chord, ChordTemplate};

#[cfg(test)]
#[test]
pub fn key_add_interval() {
	assert_eq!(Key::B, Key::A + Interval::MajorSecond);
	assert_eq!(Key::C, Key::A + Interval::Octave + Interval::MinorThird);
	assert_eq!(Key::F, Key::Cis + Interval::MajorThird);
}

#[test]
pub fn key_sub_interval() {
	assert_eq!(Key::G, Key::A - Interval::MajorSecond);
	assert_eq!(Key::Fis, Key::A - Interval::Octave - Interval::MinorThird);
}

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
