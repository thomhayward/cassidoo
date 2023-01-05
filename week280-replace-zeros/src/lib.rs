pub fn replace_zeros(s: &str) -> String {
	replace_char_with_count(s.chars(), '0').collect()
}
///
/// Returns an iterator of `char` which replaces runs of consecutive `target` chars from
/// the iterator `s` with the length of the run. All other chars are output unchanged.
///
/// Example:
/// ```
/// # use week280_replace_zeros::*;
/// let mut out = replace_char_with_count("l3aaat".chars(), 'a');
/// assert_eq!(out.next(), Some('l'));
/// assert_eq!(out.next(), Some('3'));
/// assert_eq!(out.next(), Some('3'));
/// assert_eq!(out.next(), Some('t'));
/// assert_eq!(out.next(), None);
/// ```
///
pub fn replace_char_with_count<S>(mut s: S, target: char) -> impl Iterator<Item = char>
where
	S: Iterator<Item = char>,
{
	//
	// Setup initial state for the iterator.
	//
	let mut zero_count = 0;
	let mut buffer = Vec::with_capacity(2);
	//
	//
	std::iter::from_fn(move || loop {
		//
		// Make sure our buffer is empty first
		//
		if !buffer.is_empty() {
			return buffer.pop();
		}
		//
		// The classic Rust 'loop { break match { } }' pattern. Match arms which return a value
		// will break out of the loop, which in this case will also be the return value for
		// the function. Match arms with a `continue` will continue iteration.
		//
		break match s.next() {
			Some(c) if c == target => {
				// We've encountered the target character! Increment the count and restart
				// the loop.
				//
				zero_count += 1;
				continue;
			}
			Some(c) if zero_count > 0 => {
				// We've encountered a different character at the end of a run of the target
				// character. Push the character to the buffer and then how many target
				// characters we encountered.
				//
				buffer.push(c);
				buffer.extend(zero_count.to_string().chars().rev());
				zero_count = 0;
				continue;
			}
			None if zero_count > 0 => {
				// We've reached the end of the source iterator while in a run of the target
				// character. Ensure the count gets included in the output.
				//
				buffer.extend(zero_count.to_string().chars().rev());
				zero_count = 0;
				continue;
			}
			Some(c) => {
				// We've encountered a non-target character and we know the buffer is
				// empty, so just return the character.
				Some(c)
			}
			None => None,
		};
	})
}

#[cfg(test)]
mod tests {
	use crate::replace_zeros;
	#[test]
	fn example1() {
		assert_eq!(replace_zeros("1234500362000440"), "1234523623441");
	}
	#[test]
	fn example2() {
		assert_eq!(replace_zeros("123450036200044"), "123452362344");
	}
	#[test]
	fn example3() {
		assert_eq!(replace_zeros("000000000000"), "12");
	}
	#[test]
	fn example4() {
		assert_eq!(replace_zeros("123456789"), "123456789");
	}
}
