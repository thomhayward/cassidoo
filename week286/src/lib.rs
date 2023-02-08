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
	let mut column_number = 0;
	for (position, letter) in name.chars().rev().enumerate() {
		let Some(index) = ALPHA.iter().position(|x| x == &letter) else {
			panic!("Invalid character in column name");
		};
		if position >= u32::MAX as usize {
			panic!("Column name exceeds maxiumn length")
		}
		column_number += (index + 1) * ALPHA.len().pow(position as u32);
	}
	column_number
}

const ALPHA: &[char] = &[
	'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
	'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

#[cfg(test)]
mod tests {
	use crate::column_name_to_number;
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
