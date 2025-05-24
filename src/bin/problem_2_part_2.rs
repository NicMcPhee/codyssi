use codyssi::nic_er_tools::NicErTools;
use num::Integer;

fn main() {
    let input = include_str!("../inputs/problem_2.txt");
    let median_price = even_quality_prices(input).unwrap();
    println!("The price for the even quality rooms was {median_price}.");
}

fn even_quality_prices(input: &str) -> Option<u64> {
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

    let sum_of_even_qualities: u64 = lines
        .map(|s| s.parse::<u64>().ok())
        .process_options(|i| i.filter(Integer::is_even).sum())?;

    Some(
        parameters[0]
            + parameters[1] * (sum_of_even_qualities.pow(u32::try_from(parameters[2]).ok()?)),
    )
}

#[cfg(test)]
mod tests {
    use super::even_quality_prices;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/problem_2_test.txt");
        let median_price = even_quality_prices(input).unwrap();
        assert_eq!(median_price, 1_000_986_169_836_015);
    }

    #[test]
    fn actual_input() {
        let input = include_str!("../inputs/problem_2.txt");
        let median_price = even_quality_prices(input).unwrap();
        assert_eq!(median_price, 1_653_957_213_161_029_994);
    }
}
