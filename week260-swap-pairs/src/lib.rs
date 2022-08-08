
pub fn swap_pairs<T>(s: &mut [T]) {
    for pair in s.chunks_exact_mut(2) {
        pair.swap(0, 1);
    }
}

#[ cfg(test)]
mod test {

    use super::swap_pairs;

    #[test]
    fn example() {
        let mut ex = [1, 2, 3, 4];
        swap_pairs(&mut ex);
        assert_eq!(
            ex,
            [2, 1, 4, 3]
        );
    }

    #[test]
    fn empty() {
         let mut empty: Vec<u32> = vec![];
         swap_pairs(&mut empty);
         assert_eq!(
            empty,
            []
         );
    }

}
