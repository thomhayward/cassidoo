pub fn from_to(start: usize, end: usize) -> Box<dyn FnMut() -> Option<usize>> {
	let mut range = start..=end;
	Box::new(move || range.next())
}

#[cfg(test)]
mod tests {
	use crate::from_to;

	#[test]
	fn example() {
		let mut gen = from_to(5, 7);
		assert_eq!(gen(), Some(5));
		assert_eq!(gen(), Some(6));
		assert_eq!(gen(), Some(7));
		assert_eq!(gen(), None);
	}
}
