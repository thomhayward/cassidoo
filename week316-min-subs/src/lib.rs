//! Solution to Week 316
//!
//! # Problem
//!
//! > Given an array of integers and a number k (where k is guaranteed to be
//! > less than the array's length), return a subarray of length k with the
//! > minimum possible sum. Maintain the order of the original array!

use std::iter::Sum;

/// Returns a sub-slice, of length k, from the given slice, where the sum of
/// the elements in the sub-slice is the minimum sum of all sub-slices of
/// length k.
///
/// The order of the original slice is preserved in the returned slice.
///
/// # Panics
///
/// Panics if k is greater than the length of the slice.
///
pub fn min_subs<T: Copy + Sum + Ord>(slice: &[T], k: usize) -> &[T] {
	assert!(
		k <= slice.len(),
		"k must be less than or equal to slice length"
	);

	// Return an empty slice if `k` is zero.
	if k == 0 {
		return &[];
	}

	let result = slice
		.windows(k)
		.map(|chunk| (chunk, chunk.iter().cloned().sum::<T>()))
		.min_by_key(|(_, sum)| *sum);

	// SAFETY: If `slice` is empty and `k` is non-zero, the initial assertion
	// will fail.
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
