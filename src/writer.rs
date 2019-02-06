extern crate rimd;

use std::path::Path;
use rimd::{SMFBuilder, MidiMessage, MetaEvent, SMFWriter};

use crate::rhythm::{TimePoint, RhythmNote};
use crate::note::Note;

pub struct MidiWriter {
	notes: Vec<Note>,
}

impl MidiWriter {
	pub fn new() -> MidiWriter {
		MidiWriter { notes: Vec::new() }
	}

	pub fn add_notes<'a, I>(&mut self, notes: I)
	where
		I: Iterator<Item = &'a Note>
	{
		self.notes.extend(notes);
	}

	pub fn write(&self, path: &Path) {
		// notes -> micro_timing
		let rhythm_notes: Vec<RhythmNote> = self.notes.iter().map(|note| { note.get_rhythm_note() }).collect();
		let micro_timing = RhythmNote::to_micro_timing(rhythm_notes.iter());

		// express notes as micro_timing
		let mut smf_builder: SMFBuilder = SMFBuilder::new();
		smf_builder.add_track();
		let mut last_time_point: TimePoint = TimePoint::new(0, 1);
		for note in self.notes.iter() {
			if let Some(pitch) = note.pitch.to_midi_number() {
				smf_builder.add_midi_abs(0,
										 micro_timing as u64 / note.time_point.get_denominator() as u64 * note.time_point.get_nominator() as u64,
										 MidiMessage::note_on(pitch as u8, 127, 0));

				let end_time_point = note.time_point + note.duration;
				smf_builder.add_midi_abs(0,
										 micro_timing as u64 / end_time_point.get_denominator() as u64 * end_time_point.get_nominator() as u64,
										 MidiMessage::note_off(pitch as u8, 127, 0));

				if last_time_point < end_time_point {
					last_time_point = end_time_point;
				}
			}
		}

		smf_builder.add_meta_abs(0,
								 (micro_timing as u64 / last_time_point.get_denominator() as u64 * last_time_point.get_nominator() as u64)+1,
								 MetaEvent::end_of_track());

		let mut smf = smf_builder.result();

		smf.division = micro_timing as i16;
		let writer = SMFWriter::from_smf(smf);
		writer.write_to_file(path).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::rhythm::Duration;
	use crate::keys::Key;
	use crate::pitch::{Pitch, Octave};

	#[test]
	pub fn test_writing() {
		let mut notes: Vec<Note> = Vec::new();

		let duration = Duration::quarter();

		// Alle meine Entchen
		notes.push(Note { time_point: TimePoint::new(0, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::c(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(1, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::d(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(2, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::e(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(3, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::f(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(4, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(6, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );

		notes.push(Note { time_point: TimePoint::new(8, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(9, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(10, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(11, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(12, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );

		notes.push(Note { time_point: TimePoint::new(16, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(17, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(18, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(19, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::a(), Octave { value: 4 }) } );
		notes.push(Note { time_point: TimePoint::new(20, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );

		notes.push(Note { time_point: TimePoint::new(24, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::f(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(25, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::f(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(26, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::f(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(27, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::f(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(28, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::e(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(30, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::e(), Octave { value: 3 }) } );

		notes.push(Note { time_point: TimePoint::new(32, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(33, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(34, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(35, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::g(), Octave { value: 3 }) } );
		notes.push(Note { time_point: TimePoint::new(36, 4), duration: duration, pitch: Pitch::from_key_and_octave(Key::c(), Octave { value: 3 }) } );

		let mut midi_writer = MidiWriter::new();
		midi_writer.add_notes(notes.iter());
		midi_writer.write(Path::new("./test.mid"));
	}
}
