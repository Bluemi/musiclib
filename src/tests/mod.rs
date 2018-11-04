#[allow(unused_imports)]
use crate::keys::Key;
#[allow(unused_imports)]
use crate::intervals::Interval;

#[cfg(test)]
#[test]
pub fn key_add_interval() {
	assert_eq!(Key::B, Key::A + Interval::BSecond);
	assert_eq!(Key::C, Key::A + Interval::Octave + Interval::LThird);
}

#[test]
pub fn key_sub_interval() {
	assert_eq!(Key::G, Key::A - Interval::BSecond);
	assert_eq!(Key::Fis, Key::A - Interval::Octave - Interval::LThird);
}
