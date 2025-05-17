use std::iter::once;

fn main() {
    println!("Hello world!")
}

fn compute_offset(input: &str) -> Option<i32> {
    let mut lines = input.trim().lines();
    let signs_string = lines.next_back()?;

    let values = lines
        .map(|s| s.parse().ok())
        .collect::<Option<Vec<i32>>>()?;

    // The `once()` call inserts a `+` at the start of the iterator over the sign symbols.
    // This allows us to not treat the first number in `lines` specially; we just
    // treat it as a positive value and add it in.
    let sign_chars = once('+').chain(signs_string.chars());
    let signs = sign_chars
        .map(|c| match c {
            '+' => Some(1),
            '-' => Some(-1),
            _ => None,
        })
        .collect::<Option<Vec<i32>>>()?;

    Some(signs.into_iter().zip(values).map(|(s, v)| s * v).sum())
}

#[cfg(test)]
mod tests {
    use super::compute_offset;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/problem_1_part_1_test.txt");
        let compass_offset = compute_offset(input).unwrap();
        assert_eq!(compass_offset, 21);
    }
}
