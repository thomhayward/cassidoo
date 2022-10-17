pub fn pass_doors(doors: usize, passes: usize) -> usize {
    let mut doors = vec![false; doors];
    for pass in 1..(passes + 2) {
        for (i, door) in doors.iter_mut().enumerate() {
            if i % pass == 0 {
                *door = !*door;
            }
        }
    }
    doors.iter().map(|&door| if door { 1 } else { 0 }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(pass_doors(7, 3), 4);
    }
}
