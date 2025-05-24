use std::iter::once;

use codyssi::nic_er_tools::NicErTools;

fn main() {
    let input = include_str!("../inputs/problem_1.txt");
    let compass_offset = compute_offset(input).unwrap();
    println!("The final offset was {compass_offset}.");
}

fn compute_offset(input: &str) -> Option<i32> {
    let mut lines = input.trim().lines();
    let signs_string = lines.next_back()?;

    let values = lines.map(|s| s.parse::<i32>().ok());

    // The `once()` call inserts a `+` at the start of the iterator over the sign symbols.
    // This allows us to not treat the first number in `lines` specially; we just
    // treat it as a positive value and add it in.
    let sign_chars = once('+').chain(signs_string.chars().rev());
    let signs = sign_chars.map(|c| match c {
        '+' => Some(1),
        '-' => Some(-1),
        _ => None,
    });

    signs
        .into_iter()
        .zip(values)
        .map(|(sign, value)| Some(sign? * value?))
        // Add up all the `Some` values in this iterator, returning a `None` if
        // any of the values in the iterator is `None`.`
        .process_options(|i| i.sum())
}

#[cfg(test)]
mod tests {
    use super::NicErTools;
    use super::compute_offset;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/problem_1_test.txt");
        let compass_offset = compute_offset(input).unwrap();
        assert_eq!(compass_offset, 23);
    }

    #[test]
    fn sum_of_doubled_options() {
        let result: i32 = [Some(1), Some(5), Some(10)]
            .into_iter()
            .process_options(|inner| inner.map(|x| x * 2).sum())
            .unwrap();
        assert_eq!(result, 32);
    }

    #[test]
    fn array_has_none() {
        let result: Option<i32> = [Some(1), Some(5), None, Some(10)]
            .into_iter()
            .process_options(|inner| inner.map(|x| x * 2).sum());
        assert_eq!(result, None);
    }
}
