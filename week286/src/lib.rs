///
/// Calculates the column number given an A1 reference style column name ("A", "B", .., "Z", "AA", "AB", ..).
///
/// # Example
/// ```
/// # use week286::column_name_to_number;
/// assert_eq!(column_name_to_number("A"), 1);
/// assert_eq!(column_name_to_number("AA"), 27);
/// ```
///
/// # Panics
///
/// Panics if the column name contains a character outside the range `'A'` to `'Z'`, or if the
/// column name is more than `u32::MAX` character long.
///
pub fn column_name_to_number(name: &str) -> usize {
	let mut column_index = 0;
	for (position, column_letter) in name.chars().rev().enumerate() {
		let Some(index) = ('A'..='Z').position(|x| x == column_letter).map(|x| x + 1) else {
			panic!("Column name should only include ascii A-Z");
		};
		column_index += index
			* 26_usize.pow(
				position
					.try_into()
					.expect("Column names should generally be less than ~4 billion letters long"),
			);
	}
	column_index
}

#[cfg(test)]
mod tests {
	use super::column_name_to_number;
	#[test]
	fn example1() {
		assert_eq!(column_name_to_number("A"), 1);
	}
	#[test]
	fn example2() {
		assert_eq!(column_name_to_number("B"), 2);
	}
	#[test]
	fn example3() {
		assert_eq!(column_name_to_number("C"), 3);
	}
	#[test]
	fn example4() {
		assert_eq!(column_name_to_number("Z"), 26);
	}
	#[test]
	fn example5() {
		assert_eq!(column_name_to_number("AA"), 27);
	}
	#[test]
	fn example6() {
		assert_eq!(column_name_to_number("AB"), 28);
	}
	#[test]
	fn example7() {
		assert_eq!(column_name_to_number("AAA"), 703);
	}
}
