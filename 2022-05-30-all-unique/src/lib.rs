use std::{collections::HashSet, hash::Hash};

pub fn all_unique<U: AllUnique>(a: U) -> bool {
    a.all_unique()
}

pub trait AllUnique {
    fn all_unique(self) -> bool;
}

// impl AllUnique for &str {
//     fn all_unique(self) -> bool {
//         self.char_indices().all(|(i, v)| self.rfind(v) == Some(i))
//     }
// }

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
    use super::all_unique;
    #[test]
    fn examples() {
        assert_eq!(all_unique("Cassidy".chars()), false);
        assert_eq!(all_unique("cat & dog".chars()), false);
        assert_eq!(all_unique("cat+dog".chars()), true);
    }
    #[test]
    fn integers() {
        assert_eq!(all_unique([1, 2, 3, 4, 5, 6, 7, 8, 9]), true);
        assert_eq!(all_unique([1, 2, 3, 4, 5, 5, 6, 7, 8]), false);
    }
    #[test]
    fn strings() {
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
    #[test]
    fn infinite_fourty_twos() {
        // constructor an `Iterator` just spits out `42`
        let fourty_twos = std::iter::from_fn(|| Some(42));
        assert_eq!(all_unique(fourty_twos), false);
    }
    #[test]
    fn empty() {
        assert_eq!(all_unique(Vec::<u32>::new()), true);
    }
}
