fn main() {
    let input = include_str!("../inputs/problem_2.txt");
    let median_price = median_price(input).unwrap();
    println!("The price for a median-quality room was {median_price}.");
}

fn median_price(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    // The parameters will be the values corresponding to the
    // last word in each of these lines:
    //
    // Function A: ADD 495
    // Function B: MULTIPLY 55
    // Function C: RAISE TO THE POWER OF 3
    //
    // We apply these functions in reverse (C, then B, then A).
    let parameters = lines
        .by_ref()
        .take(3)
        .map(|s| s.split_ascii_whitespace().last()?.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?;
    assert_eq!(3, parameters.len());

    // Drop the blank line
    lines.next();

    let mut qualities = lines
        .map(|s| s.parse::<u64>().ok())
        .collect::<Option<Vec<_>>>()?;

    let middle_position = qualities.len() / 2;
    let (_, middle, _) = qualities.select_nth_unstable(middle_position);

    Some(parameters[0] + parameters[1] * (middle.pow(u32::try_from(parameters[2]).ok()?)))
}

#[cfg(test)]
mod tests {
    use super::median_price;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/problem_2_test.txt");
        let median_price = median_price(input).unwrap();
        assert_eq!(median_price, 9_130_674_516_975);
    }

    #[test]
    fn actual_input() {
        let input = include_str!("../inputs/problem_2.txt");
        let median_price = median_price(input).unwrap();
        assert_eq!(median_price, 9_563_387_078_987);
    }
}
