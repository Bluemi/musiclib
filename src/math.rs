pub fn neg_modulo(value: i32, divider: u32) -> u32
{
	if value < 0 {
		let multi: u32 = (value.abs() as u32) / divider + 1;
		((value + (multi * divider) as i32) as u32) % divider
	} else {
		((value as u32) % divider)
	}
}

pub fn least_common_multiple(value1: i32, value2: i32) -> i32 {
	if (value1 == 0) && (value2 == 0) { return 0; }

	return (value1.abs() / greatest_common_divisor(value1, value2)) * value2.abs();
}

#[allow(dead_code)]
pub fn least_common_multiple_of_vec(vec: &Vec<i32>) -> Result<i32, &str> {
	if vec.len() < 2 {
		return Err("vec should at least contain 2 values");
	}
	let mut value: i32 = 1;
	for v in vec {
		if *v == 0 {
			return Err("vec should not contain 0");
		}
		value = least_common_multiple(value, *v);
	}
	Ok(value)
}

pub fn greatest_common_divisor(value1: i32, value2: i32) -> i32 {
	if value1 == 0 { return value2; }
	if value2 == 0 { return value1; }

	return greatest_common_divisor(value2, value1 % value2);
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

	#[test]
	pub fn test_gcd() {
		assert_eq!(greatest_common_divisor(5, 15), 5);
		assert_eq!(greatest_common_divisor(9, 21), 3);
	}

	#[test]
	pub fn test_lcm() {
		assert_eq!(least_common_multiple(5, 15), 15);
		assert_eq!(least_common_multiple(9, 21), 63);
		assert_eq!(least_common_multiple(24, 7), 168);

		assert_eq!(least_common_multiple_of_vec(&vec!(3, 5, 8, 35)), Ok(840));
		assert_eq!(least_common_multiple_of_vec(&vec!(4, 7, 1, 3)), Ok(84));
		assert_eq!(least_common_multiple_of_vec(&vec!(4, 0)), Err("vec should not contain 0"));
		assert_eq!(least_common_multiple_of_vec(&vec!(4)), Err("vec should at least contain 2 values"));
	}
}
