pub fn pass_doors(doors: usize, passes: usize) -> usize {
    let mut doors = vec![true; doors];
    for pass in 2..(passes + 2) {
        for door in doors.iter_mut().step_by(pass) {
            *door = !*door;
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
