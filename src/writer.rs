extern crate rimd;

use std::path::Path;
use rimd::{SMFBuilder, SMFWriter, MidiMessage, MetaEvent};

use crate::note::Note;

pub struct MidiWriter {
	sfm_builder: SFMBuilder,
}

impl MidiWriter {
	pub fn new() -> MidiWriter {
		let mut sfm_builder = SFMBuilder::new();
		sfm_builder.add_track();
		MidiWriter { sfm_builder }
	}

	pub fn write_bar(&mut self, notes: &Vec<Note>) {
		for note in notes {
			self.sfm_builder.add_midi_abs(0, 
		}
	}
}
