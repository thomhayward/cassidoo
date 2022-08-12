///
/// An iterator that yields elements from another iterator with every consecutive pair of elements swapped. If the source iterator
/// has an odd number of elements, the final element is yielded unmodified.
/// 
/// # Example
/// 
/// ```
/// # use week260_swap_pairs::iters::*;
/// let values = vec!['o', 'l', 'e', 'r', 'm'];
/// let swapped = values.swap_pairs().collect::<Vec<_>>();
/// assert_eq!(swapped, &['l', 'o', 'r', 'e', 'm']);
/// ```
/// 
pub struct PairSwap<I, T>
where
    I: Iterator<Item = T>,
{
    iterator: I,
    // A stack of 0 or 1 items.
    stack: Option<I::Item>,
}

impl<I, T> Iterator for PairSwap<I, T>
where
    I: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_some() {
            // set `stack` to `None` and return the previous value of `stack`.
            std::mem::replace(&mut self.stack, None)
        } else {
            // Get the next pair from the source iterator
            match (self.iterator.next(), self.iterator.next()) {
                (Some(a), Some(b)) => {
                    self.stack = Some(a);
                    Some(b)
                }
                (Some(a), None) => Some(a),
                (None, _) => None
            }
        }
    }
}

pub trait SwapPairs<I, T>
where
    I: Iterator<Item = T>,
{
    fn swap_pairs(self) -> PairSwap<I, T>;
}

impl<J, I, T> SwapPairs<I, T> for J
where
    J: IntoIterator<Item = T> + IntoIterator<IntoIter = I>,
    I: Iterator<Item = T>,
{
    fn swap_pairs(self) -> PairSwap<I, T> {
        PairSwap {
            iterator: self.into_iter(),
            stack: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let values = [1, 2, 3, 4];
        assert_eq!(values.swap_pairs().collect::<Vec<_>>(), &[2, 1, 4, 3]);
    }
    #[test]
    fn example_odd() {
        let values = vec![1, 2, 3, 4, 5];
        assert_eq!(values.swap_pairs().collect::<Vec<_>>(), &[2, 1, 4, 3, 5]);
    }
    #[test]
    fn powers_of_two() {
        let mut i: u32 = 0;
        let powers_of_two = std::iter::from_fn(move || {
            i += 1;
            Some(2_u64.pow(i))
        });
        assert_eq!(
            powers_of_two.take(16).swap_pairs().collect::<Vec<_>>(),
            &[4, 2, 16, 8, 64, 32, 256, 128, 1024, 512, 4096, 2048, 16384, 8192, 65536, 32768]
        );
    }
    #[test]
    fn example_linked_list() {
        let ll = std::collections::LinkedList::from([1, 2, 3, 4]);
        assert_eq!(ll.swap_pairs().collect::<Vec<_>>(), &[2, 1, 4, 3]);
    }
}
