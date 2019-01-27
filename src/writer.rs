extern crate rimd;

// use std::path::Path;
// use rimd::{SMFBuilder, SMFWriter, MidiMessage, MetaEvent};

use crate::rhythm::RhythmNote;
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

    pub fn write(&self) {
        // notes -> micro_timing
        let rhythm_notes: Vec<RhythmNote> = self.notes.iter().map(|note| { note.get_rhythm_note() }).collect();
        let micro_timing = RhythmNote::to_micro_timing(rhythm_notes.iter());

        // express notes as micro_timing
    }
}
