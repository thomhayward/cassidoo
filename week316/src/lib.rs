//! Solution to Week 316
//!
//! # Problem
//!
//! Given an array of integers and a number k (where k is guaranteed to be
//! less than the array's length), return a subarray of length k with the
//! minimum possible sum. Maintain the order of the original array!

/// Finds the sub-slice of `slice`, with `k` elements, which has the minimum
/// sum of all sub-slices of length `k`
///
/// The order of the array is preserved in the returned slice.
///
/// # Panics
///
/// Panics if `k` is greater than the length of `slice`.
///
pub fn min_subs(slice: &[u32], k: usize) -> &[u32] {
	assert!(
		k <= slice.len(),
		"k must be less than or equal to slice length"
	);

	if k == 0 {
		return &[];
	}

	let result = slice
		.windows(k)
		.map(|chunk| (chunk, chunk.iter().sum::<u32>()))
		.min_by_key(|(_, sum)| *sum);

	// This unwrap can never panic. If `slice` is empty the assertion will
	// fail unless `k` is 0, in which case `.windows()` will panic instead.
	let (min_sub, _) = unsafe { result.unwrap_unchecked() };
	min_sub
}

#[cfg(test)]
mod tests {
	use super::min_subs;

	#[test]
	fn example1() {
		#[rustfmt::skip]
		assert_eq!(
			min_subs([1, 3, 20, 4, 8, 9, 11].as_slice(), 3),
			[4, 8, 9]
		);
	}

	#[test]
	fn example2() {
		#[rustfmt::skip]
		assert_eq!(
			min_subs([4, 4, 4, 4, 8].as_slice(), 2),
			[4, 4]
		);
	}

	#[test]
	fn zero_k() {
		#[rustfmt::skip]
		assert_eq!(
			min_subs(&[1, 2, 3], 0),
			[]
		);
	}

	#[test]
	#[should_panic]
	fn k_too_large() {
		min_subs(&[1, 2, 3], 4);
	}
}