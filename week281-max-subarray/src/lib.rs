use std::iter::Sum;
///
/// Returns the sub-slice of `arr` of length `n` with the largest sum of values.
///
/// An empty slice is returned if `n` is `0` or greater than the length of `arr`.
///
pub fn max_subarray<'a, T>(arr: &'a [T], n: usize) -> &[T]
where
    T: Copy + Ord + Sum<&'a T> + 'a,
{
    if n == 0 || n > arr.len() {
        &[]
    } else {
        &arr.windows(n)
            .map(|window| (window, window.iter().sum::<T>()))
            .max_by_key(|(_, sum)| *sum)
            .map(|(window, _)| window)
            .unwrap_or(&[])
    }
}

#[cfg(test)]
mod tests {
    use super::max_subarray;
    #[test]
    fn example1() {
        assert_eq!(
            max_subarray(&[-4, 2, -5, 1, 2, 3, 6, -5, 1], 4),
            [1, 2, 3, 6]
        );
    }
    #[test]
    fn example2() {
        assert_eq!(max_subarray(&[1, 2, 0, 5], 2), [0, 5])
    }
    #[test]
    fn zero_length() {
        assert_eq!(max_subarray(&[1, 2, 3, 4], 0), &[]);
    }
    #[test]
    fn insufficient_items() {
        assert_eq!(max_subarray(&[1, 2], 4), []);
    }
}
