pub fn neg_modulo(value: i32, divider: u32) -> u32
{
	if value < 0 {
		let multi: u32 = (value.abs() as u32) / divider + 1;
		((value + (multi * divider) as i32) as u32) % divider
	} else {
		((value as u32) % divider)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	pub fn test_neg_modulo() {
		assert_eq!(neg_modulo(2, 10), 2);
		assert_eq!(neg_modulo(-2, 10), 8);
		assert_eq!(neg_modulo(-13, 10), 7);
	}
}
