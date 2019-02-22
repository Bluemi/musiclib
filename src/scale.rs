use crate::keys::Key;
use crate::intervals::Interval;
use crate::pitch::Pitch;

pub struct ScaleTemplate {
	pub intervals: Vec<Interval>,
}

pub struct KeyScale {
	pub keys: Vec<Key>,
}

pub struct PitchScale {
	pub pitches: Vec<Pitch>,
}

impl ScaleTemplate {
	pub fn new() -> ScaleTemplate {
		ScaleTemplate { intervals: Vec::new() }
	}

	pub fn chromatic_scale() -> ScaleTemplate {
		let intervals = vec![Interval { halftones: 0 },
							 Interval { halftones: 1 },
							 Interval { halftones: 2 },
							 Interval { halftones: 3 },
							 Interval { halftones: 4 },
							 Interval { halftones: 5 },
							 Interval { halftones: 6 },
							 Interval { halftones: 7 },
							 Interval { halftones: 8 },
							 Interval { halftones: 9 },
							 Interval { halftones: 10 },
							 Interval { halftones: 11 }];
		ScaleTemplate { intervals: intervals }
	}

	pub fn major() -> ScaleTemplate {
		let intervals = vec![Interval { halftones: 0 },
							 Interval { halftones: 2 },
							 Interval { halftones: 4 },
							 Interval { halftones: 5 },
							 Interval { halftones: 7 },
							 Interval { halftones: 9 },
							 Interval { halftones: 11 }];
		ScaleTemplate { intervals: intervals }
	}

	pub fn minor() -> ScaleTemplate {
		let intervals = vec![Interval { halftones: 0 },
							 Interval { halftones: 2 },
							 Interval { halftones: 3 },
							 Interval { halftones: 5 },
							 Interval { halftones: 7 },
							 Interval { halftones: 8 },
							 Interval { halftones: 10 }];
		ScaleTemplate { intervals }
	}

	pub fn minor_harmonic() -> ScaleTemplate {
		let intervals = vec![Interval { halftones: 0 },
							 Interval { halftones: 2 },
							 Interval { halftones: 3 },
							 Interval { halftones: 5 },
							 Interval { halftones: 7 },
							 Interval { halftones: 8 },
							 Interval { halftones: 11 }];
		ScaleTemplate { intervals }
	}
}

impl KeyScale {
	pub fn from_scale_template(scale_template: &ScaleTemplate, keynote: Key) -> KeyScale {
		let keys = scale_template.intervals.iter().map(|interval| keynote + *interval).collect();
		KeyScale { keys }
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn major_scales() {
		let major_a_scale = KeyScale::from_scale_template(&ScaleTemplate::major(), Key::a());
		let major_a_keys = vec![Key::a(), Key::b(), Key::cis(), Key::d(), Key::e(), Key::fis(), Key::gis()];
		assert_eq!(major_a_scale.keys, major_a_keys);

		let major_cis_scale = KeyScale::from_scale_template(&ScaleTemplate::major(), Key::cis());
		let major_cis_keys = vec![Key::cis(), Key::dis(), Key::f(), Key::fis(), Key::gis(), Key::ais(), Key::c()];
		assert_eq!(major_cis_scale.keys, major_cis_keys);
	}

	#[test]
	pub fn minor_scales() {
		let minor_a_scale = KeyScale::from_scale_template(&ScaleTemplate::minor(), Key::a());
		let minor_a_keys = vec![Key::a(), Key::b(), Key::c(), Key::d(), Key::e(), Key::f(), Key::g()];
		assert_eq!(minor_a_scale.keys, minor_a_keys);

		let minor_c_scale = KeyScale::from_scale_template(&ScaleTemplate::minor(), Key::c());
		let minor_c_keys = vec![Key::c(), Key::d(), Key::dis(), Key::f(), Key::g(), Key::gis(), Key::ais()];
		assert_eq!(minor_c_scale.keys, minor_c_keys);

		// harmonic minor
		let minor_harmonic_a_scale = KeyScale::from_scale_template(&ScaleTemplate::minor_harmonic(), Key::a());
		let minor_harmonic_a_keys = vec![Key::a(), Key::b(), Key::c(), Key::d(), Key::e(), Key::f(), Key::gis()];
		assert_eq!(minor_harmonic_a_scale.keys, minor_harmonic_a_keys);
	}
}
