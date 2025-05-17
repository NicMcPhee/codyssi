use std::iter::once;

fn main() {
    let input = include_str!("../inputs/problem_1_part_1.txt");
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
    let sign_chars = once('+').chain(signs_string.chars());
    let signs = sign_chars.map(|c| match c {
        '+' => Some(1),
        '-' => Some(-1),
        _ => None,
    });

    signs
        .into_iter()
        .zip(values)
        .map(|(s, v)| Some(s? * v?))
        .process_options(|i| i.sum())
}

struct ProcessOptions<'a, OptionIterator> {
    internal_iterator: OptionIterator,
    found_none: &'a mut bool,
}

impl<OptionIterator, T> Iterator for ProcessOptions<'_, OptionIterator>
where
    OptionIterator: Iterator<Item = Option<T>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.found_none {
            None
        } else {
            let n = self.internal_iterator.next();
            match n {
                Some(None) => {
                    *self.found_none = true;
                    None
                }
                Some(Some(v)) => Some(v),
                None => None,
            }
        }
    }
}

trait NicErTools {
    fn process_options<F, T, R>(self, processor: F) -> Option<R>
    where
        Self: Iterator<Item = Option<T>>,
        F: FnOnce(ProcessOptions<'_, Self>) -> R;
}

impl<I> NicErTools for I
where
    I: Iterator,
{
    fn process_options<F, T, R>(self, processor: F) -> Option<R>
    where
        Self: Iterator<Item = Option<T>>,
        F: FnOnce(ProcessOptions<'_, Self>) -> R,
    {
        let mut found_none: bool = false;

        let process_options = ProcessOptions {
            internal_iterator: self,
            found_none: &mut found_none,
        };

        let result = processor(process_options);

        if found_none { None } else { Some(result) }
    }
}

#[cfg(test)]
mod tests {
    use super::NicErTools;
    use super::compute_offset;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/problem_1_part_1_test.txt");
        let compass_offset = compute_offset(input).unwrap();
        assert_eq!(compass_offset, 21);
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
