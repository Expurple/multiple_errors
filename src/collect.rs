/// Provides `collect_vec_result() -> Result<Vec<T>, Vec<E>>`
pub trait CollectVecResult: Iterator<Item = Result<Self::T, Self::E>> {
    type T;
    type E;
    /// Like standard `Iterator::collect::<Result<Vec<T>, E>>`, but all errors
    /// instead of just the first one.
    ///
    /// Doesn't short-circuit, always exhausts the iterator.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use multiple_errors::testing_prelude::*;
    /// # use multiple_errors::CollectVecResult;
    /// #
    /// assert_eq!(
    ///     [Err(ErrA), Ok(A), Err(ErrA)].into_iter().collect_vec_result(),
    ///     // Collected all errors, not just the first one
    ///     Err(vec![ErrA, ErrA])
    /// );
    /// ```
    fn collect_vec_result(self) -> Result<Vec<Self::T>, Vec<Self::E>>;
}

impl<I, T, E> CollectVecResult for I
where
    I: Iterator<Item = Result<T, E>>,
{
    type T = T;
    type E = E;
    fn collect_vec_result(self) -> Result<Vec<Self::T>, Vec<Self::E>> {
        let mut oks = vec![];
        let mut errs = vec![];
        for elem in self {
            match elem {
                Err(err) => errs.push(err),
                Ok(ok) => {
                    // Don't grow `oks` if we already know that we'll discard it.
                    if errs.is_empty() {
                        oks.push(ok)
                    }
                }
            }
        }
        match errs.as_slice() {
            [] => Ok(oks),
            [_, ..] => Err(errs),
        }
    }
}
