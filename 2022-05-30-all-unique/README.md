# All Unique

> "Write a function that determines if all the characters in a given string are unique."

TLDR:

```rust
pub fn all_unique(s: &str) -> bool {
    s.chars().enumerate().all(|(i, c)| s.rfind(c) == Some(i))
}

#[cfg(test)]
mod tests {
    use super::all_unique;
    #[test]
    fn examples() {
        assert_eq!(all_unique("Cassidy"), false);
        assert_eq!(all_unique("cat & dog"), false);
        assert_eq!(all_unique("cat+dog"), true);
    }
}
```

Yay! Basically a one-line solution to an interview problem!

Note that we don't allocate any new variables. The functions we call *do* but *we* don't. Want to ignore capitalisation? Slam a `.to_lowercase()` on the end of your string. Why would I build that into my API? Srsly.

Interestingly, one can solve this problem almost identically in Javascript:

```js
function allUnique(str) {
    return Array.from(str).every((char, index, arr) => arr.indexOf(char) === index);
}
```

While our Rust solution will only work on an `&str` this Javascript function will work for any Array-like input. So let's see how we can make the Rust solution a bit more flexible.

First up, lets define a trait and modify the `all_unique()` function to use it &hellip;

```rust
pub trait AllUnique {
    fn all_unique(self) -> bool;
}

pub fn all_unique<U: AllUnique>(a: U) -> bool {
    a.all_unique()
}
```

&hellip; and implement the new trait for `&str`:

```rust
impl AllUnique for &str {
    fn all_unique(self) -> bool {
        self.chars().enumerate().all(|(i, v)| self.rfind(v) == Some(i))
    }
}
```

Cool beans. We've written a whole bunch more code to do the same thing. Now let's make it work over anything iterable. Time to deploy [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html).

```rust
impl<I, T> AllUnique for I
where
    I: IntoIterator<Item = T>,
    T: PartialEq,
{
    fn all_unique(self) -> bool {
        let vec = Vec::from_iter(self);
        vec.iter().enumerate().all(|(i, v)| vec.iter().rposition(|x| x == v) == Some(i))
    }
}
```

Now we can call `all_unique()` for anything that can be turning into an `Iterator` over any type that implements `PartialEq` (needed for `==`).

Note that we can no longer just shove a `&str` into `all_unique()`. This is not unreasonable. We can treat a `&str` as a sequence of Unicode `char`'s or as a sequence of raw `u8` bytes, and deciding which is the caller's job not ours. Time to adjust the test suite:

```rust
#[cfg(test)]
mod tests {
    use super::all_unique;
    #[test]
    fn examples() {
        assert_eq!(all_unique("Cassidy".chars()), false);
        assert_eq!(all_unique("cat & dog".chars()), false);
        assert_eq!(all_unique("cat+dog".chars()), true);
    }
}
```

Not bad DX. Use `.bytes()` instead to compare the raw representation. We could also add some more tests to prove a point.

```rust
#[cfg(test)]
mod tests {
    // ... the other tests ...
    #[test]
    fn integers() {
        assert_eq!(all_unique([1, 2, 3, 4, 5, 6, 7, 8, 9]), true);
        assert_eq!(all_unique([1, 2, 3, 4, 5, 5, 6, 7, 8]), false);
    }
    #[test]
    fn vec_of_strings() {
        let mut chars = vec!["Spock", "Kirk", "Uhura", "Red-Shirt"];
        assert_eq!(all_unique(&chars), true);
        chars.push("Red-Shirt");
        assert_eq!(all_unique(&chars), false);
    }
    #[test]
    fn more_strings() {
        assert_eq!(
            all_unique("Can you spot the the obvious error?".split(" ")),
            false
        );
        assert_eq!(
            all_unique("Every word in this sentence is unsurprisingly unique".split(" ")),
            true
        );
    }
}
```

With this `IntoIterator` implementation we cannot consume the iterable more than once, so we build a `Vec` from the iterable and then enumerate the elements of the `Vec`. If we've resigned ourselves to allocating memory anyway, why not just use a [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html)?

```rust
impl<I, T> AllUnique for I
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    fn all_unique(self) -> bool {
        let mut count = 0;
        let iter = self.into_iter().map(|item| {
            count += 1;
            item
        });
        HashSet::<T>::from_iter(iter).len() == count
    }
}
```

Bangin'. Now we're basically running in _O(n)_, and we can run `all_unique()` over any iterable type which implements `Hash` and `Eq`. What if we want to test an infinite iterator? We're currently constructing a `HashSet` from the whole iterable source. Time for some tweaking:

```rust
impl<I, T> AllUnique for I
where
    I: IntoIterator<Item = T>,
    T: Eq + Hash,
{
    fn all_unique(self) -> bool {
        let mut set = HashSet::new();
        for item in self.into_iter() {
            match set.contains(&item) {
                true => return false,
                false => set.insert(item),
            };
        }
        true
    }
}

#[cfg(test)]
mod tests {
    // ... the other tests ...
    #[test]
    fn infinite_fourty_twos() {
        // constructor an `Iterator` just spits out `42`
        let fourty_twos = std::iter::from_fn(|| Some(42));
        assert_eq!(all_unique(fourty_twos), false);
    }
}
```

Boom! Short-circuiting! Now `all_unique()` will run until the iterator spits out a non-unique element &hellip; or the computer runs out of memory. Neat.

TODO: Some types don't implement `Hash` or `Eq`. We can't currently use `all_unique` on `f32`'s or `f64`'s. We've already exceeded the scope of the problem, so we'll just gloss over that issue.
