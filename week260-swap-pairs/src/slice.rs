pub trait SwapPairs {
    fn swap_pairs(self);
}

impl<T> SwapPairs for &mut [T] {
    #[inline(always)]
    fn swap_pairs(self) {
        for pair in self.chunks_exact_mut(2) {
            pair.swap(0, 1);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example() {
        let mut input = [1, 2, 3, 4];
        input.swap_pairs();
        assert_eq!(input, [2, 1, 4, 3]);
    }
    #[test]
    fn unpaired_element() {
        let mut input = vec![1, 2, 3, 4, 5];
        // Swap pairs of elements in `input`, leaving any remaining unpaired element at the end
        input.swap_pairs();
        assert_eq!(input, &[2, 1, 4, 3, 5]);
    }
    #[test]
    fn empty() {
        let mut empty: Vec<u32> = vec![];
        empty.swap_pairs();
        assert_eq!(empty, &[]);
    }
}
