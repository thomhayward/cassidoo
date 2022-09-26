///
/// Returns a string representation of `n` and its ordinal suffix concatenated together.
/// 
/// Example:
/// ```
/// # use week267_ordinal_suffix::ordinal;
/// assert_eq!(ordinal(3), "3rd");
/// assert_eq!(ordinal(57), "57th");
/// ```
pub fn ordinal(n: u64) -> String {
    format!("{}{}", n, ordinal_indicator(n))
}
///
/// Returns the English ordinal indicator for the unsigned integer `n`.
///
/// Example:
/// ```
/// # use week267_ordinal_suffix::ordinal_indicator;
/// assert_eq!(ordinal_indicator(1), "st");
/// assert_eq!(ordinal_indicator(2), "nd");
/// assert_eq!(ordinal_indicator(12), "th");
/// ```
pub fn ordinal_indicator(n: u64) -> &'static str {
    let n = format!("{}", n);
    let mut s = n.chars().rev();
    match (s.next(), s.next()) {
        (Some('1'), Some('1'))      // numbers ending with "11" -> "11th".
        | (Some('2'), Some('1'))    // numbers ending with "12" -> "12th".
        | (Some('3'), Some('1'))    // numbers ending with "13" -> "13th".
        | (Some('4'), _)
        | (Some('5'), _)
        | (Some('6'), _)
        | (Some('7'), _)
        | (Some('8'), _)
        | (Some('9'), _)
        | (Some('0'), _) => "th",
        (Some('1'), _) => "st",
        (Some('2'), _) => "nd",
        (Some('3'), _) => "rd",
        (_, _) => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::ordinal;
    #[test]
    fn the_basics() {
        assert_eq!(ordinal(1), "1st");
        assert_eq!(ordinal(2), "2nd");
        assert_eq!(ordinal(3), "3rd");
        assert_eq!(ordinal(4), "4th");
        assert_eq!(ordinal(5), "5th");
        assert_eq!(ordinal(6), "6th");
        assert_eq!(ordinal(7), "7th");
        assert_eq!(ordinal(8), "8th");
        assert_eq!(ordinal(9), "9th");
        assert_eq!(ordinal(10), "10th");
    }
    #[test]
    fn zeros() {
        assert_eq!(ordinal(0), "0th");
        assert_eq!(ordinal(10), "10th");
        assert_eq!(ordinal(100), "100th");
        assert_eq!(ordinal(1000), "1000th");
    }
    #[test]
    fn ths() {
        assert_eq!(ordinal(11), "11th");
        assert_eq!(ordinal(12), "12th");
        assert_eq!(ordinal(13), "13th");
        //
        assert_eq!(ordinal(21), "21st");
        assert_eq!(ordinal(22), "22nd");
        assert_eq!(ordinal(23), "23rd");
    }
    #[test]
    fn bigly_numbers() {
        assert_eq!(ordinal(111), "111th");
        assert_eq!(ordinal(151), "151st");
        assert_eq!(ordinal(2001), "2001st");
        assert_eq!(ordinal(112111), "112111th");
        assert_eq!(ordinal(112161), "112161st");
        assert_eq!(ordinal(u64::MAX), "18446744073709551615th");
    }
}
