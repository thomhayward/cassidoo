# Week 260 - Swap Pairs

> "__Given a list, swap every two adjacent nodes.__ Something to think about: How would your code change if this were a linked list, versus an array?"

Example:
```js
> swapPairs([1, 2, 3, 4])
> [2, 1, 4, 3]

> swapPairs([])
> []
```

TLDR:
```rust
fn swap_pairs<T>(slice: &mut &[T]) {
    for pair in slice.chunks_exact_mut(2) {
        pair.swap(0, 1);
    }
}
```

Neat! Problem solved.

While this is cool it's a bit limiting in that it can only work on mutatable slices (`&mut [T]`), and not all collections are sliceable, and we don't always want to stomp all over our inputs. Instead we can write a version which can consume any iterable type. To create our own iterator, we first need to define a struct:

```rust
struct PairSwap<I, T> where I: Iterator<Item = T> {
    iterator: I,
    stack: Option<I::Item> // a stack of 0 or 1 items
}
```

Now we need to implement the trait `Iterator` for `PairSwap`:

```rust
impl<I, T> Iterator for PairSwap<I, T> where I: Iterator<Item = T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // determine if the 'stack' has an item in it from a previous iteration --
        if self.stack.is_none() {
            // -- it does not!
            // attempt to get two elements from the source iterator
            match (self.iterator.next(), self.iterator.next()) {
                (Some(a), Some(b)) => {
                    // we have a pair, save the first element in the 'stack' for the next iteration
                    self.stack = Some(a);
                    // yield the second element.
                    Some(b)
                }
                (Some(a), None) => {
                    // we only received one element from the iterator. The source iterator must have an odd number
                    // of elements. Yield it!
                    Some(a)
                },
                (None, _) => {
                    // the source iterator has run out of elements to yield, which means we have run out of
                    // elements to yield as well.
                    None
                }
            }
        } else {
            // -- it does!
            // empty the stack and yield the value that was in it.
            std::mem::replace(&mut self.stack, None)
            // nb.
            // we cannot write:
            //      let temp = self.stack;
            //      self.stack = None
            //      temp
            // because this would make the borrow checker very cross
        }
    }
}
```

To use:

```rust
let values = vec![1, 2, 3, 4, 5, 6];
let swapper = PairSwap {
    iterator: values.into_iter(),
    stack: None,
};
for element in swapper {
    print!("{} ", element);
}
println!("");
```

Would output:
```
2 1 4 3 6 5
```

We can tidy up the DX with a trait.

```rust
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
```

Now for any type which implements `IntoIterator` we can call `.swap_pairs()` on it.

So the previous usage example becomes:

```rust
let values = vec![1, 2, 3, 4, 5, 6];
for element in values.swap_pairs() {
    print!("{} ", element);
}
```

```
2 1 4 3 6 5
```

Or maybe even:

```rust
use std::collections::LinkedList;
let linked_list = LinkedList::from([1, 2, 3, 4, 5, 6]);
for element in linked_list.swap_pairs() {
    ...
}
```
