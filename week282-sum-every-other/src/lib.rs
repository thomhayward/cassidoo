///
/// Returns the sum of every other digit ('0'-'9') in the `String`-serialised form of `s`.
///
pub fn sum_every_other(s: impl ToString) -> u32 {
	s.to_string()
		.chars()
		.filter(|c| c.is_digit(10))
		.skip(1)
		.step_by(2)
		.map(|c| format!("{c}").parse::<u32>().unwrap())
		.sum()
}

#[cfg(test)]
mod tests {
	use super::sum_every_other;
	#[test]
	fn example1() {
		assert_eq!(sum_every_other(548915381), 26);
	}
	#[test]
	fn example2() {
		assert_eq!(sum_every_other(10), 0);
	}
	#[test]
	fn example3() {
		assert_eq!(sum_every_other(1010.11), 1);
	}
}
