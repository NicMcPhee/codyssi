// To process an iterator of `Option`s we'll need to have
// the iterator that provides those `Option` values, and
// mutable access to a boolean that keeps track of whether
// we encountered a `None` value in that sequence of
// `Option`s.
pub struct ProcessOptions<'a, OptionIterator> {
    internal_iterator: OptionIterator,
    found_none: &'a mut bool,
}

pub fn process_options<F, T, R, I>(internal_iterator: I, processor: F) -> Option<R>
where
    I: Iterator<Item = Option<T>>,
    F: FnOnce(ProcessOptions<'_, I>) -> R,
{
    // This keeps track of whether any of the values in `Self`
    // had the value `None`.
    let mut found_none: bool = false;

    let process_options = ProcessOptions {
        internal_iterator,
        found_none: &mut found_none,
    };

    let result = processor(process_options);

    // If we ever encountered a `None` value, we want
    // to return `None`; otherwise we wrap `result` in
    // a `Some` variant.
    if found_none { None } else { Some(result) }
}

// Implement `Iterator` for our `ProcessOptions` structure. This
// mostly just passes the work on to the `internal_iterator`, along
// with a bunch of "wrapper" logic to keep track of whether we've
// encountered a `None` value.
impl<OptionIterator, T> Iterator for ProcessOptions<'_, OptionIterator>
where
    OptionIterator: Iterator<Item = Option<T>>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if *self.found_none {
            // If we've ever encountered a `None` value, we should
            // always return `None` for subsequent requests.
            None
        } else {
            // Get the next value from the internal iterator.
            let n = self.internal_iterator.next();
            match n {
                // We've reached the end of the `internal_iterator`, so
                // we return `None` to indicate that the iterator is
                // finished.
                None => None,
                // The `internal_iterator` returned a `None` _value_
                // (i.e., wrapped in a `Some`), which means that we
                // want to record this in `found_none` and return
                // `None`.
                Some(None) => {
                    *self.found_none = true;
                    None
                }
                // The `internal_iterator` returned a `Some` value
                // so we want to pass that along.
                Some(Some(v)) => Some(v),
            }
        }
    }
}

// TODO: Implement `DoubleEndedIterator`
// TODO: Implement `FusedIterator`
