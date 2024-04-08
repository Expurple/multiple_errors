/// If at least one `Result` is an error, turn all of them into errors. Else, unwrap the `Result`s.
///
/// See [fail_all_vec] for an easier-to-understand version specialized for `Vec`.
///
/// [fail_all] is its lower-level implementation. You probably want to create
/// a similar wrapper function instead of using [fail_all] directly.
///
/// ## Examples
///
/// ```rust
/// # use multiple_errors::fail_all;
/// # use multiple_errors::testing_prelude::*;
/// #
/// // Manually:
///
/// let Ok(ok) = fail_all([Ok(A), Ok(A)], |_: Result<_, ErrA>| ErrC) else {
///     panic!();
/// };
/// assert_eq!(ok.collect::<Vec<_>>(), vec![A, A]);
///
/// // Or using a helper function for your container:
///
/// pub fn fail_all_3<T, E1, E2, F>(results: [Result<T, E1>; 3], f: F) -> Result<[T; 3], [E2; 3]>
/// where
///     F: FnMut(Result<T, E1>) -> E2,
/// {
///     fn collect_3<T>(mut iter: impl Iterator<Item = T>) -> [T; 3] {
///         core::array::from_fn(|_| iter.next().expect("the iterator should have 3 elements"))
///     }
///     fail_all(results, f).map(collect_3).map_err(collect_3)
/// }
///
/// let err = fail_all_3(
///     [Ok(A), Err(ErrA), Ok(A)],
///     |res| res.err().map(HighLevelErr::from).unwrap_or(HighLevelErr::B(ErrB))
/// );
/// assert_eq!(err, Err([ErrB.into(), ErrA.into(), ErrB.into()]));
/// ```
pub fn fail_all<I, T, E1, E2, F>(
    results: I,
    f: F,
) -> Result<impl Iterator<Item = T>, impl Iterator<Item = E2>>
where
    I: IntoIterator<Item = Result<T, E1>>,
    for<'a> &'a I: IntoIterator<Item = &'a Result<T, E1>>,
    F: FnMut(Result<T, E1>) -> E2,
{
    if (&results).into_iter().any(Result::is_err) {
        return Err(results.into_iter().map(f));
    }
    Ok(results.into_iter().map(
        // This was going to be an `expect()`, but it requires an easily avoidable `E1: Debug`.
        |res| {
            res.ok()
                .unwrap_or_else(|| panic!("errors should be handled in the previous branch"))
        },
    ))
}

/// If at least one `Result` is an error, turn all of them into errors. Else, unwrap the `Result`s.
///
/// See [fail_all] for a more generic/low-level version that works with other
/// input containers and doesn't `collect()`.
///
/// ## Examples
///
/// ```rust
/// # use multiple_errors::fail_all_vec;
/// # use multiple_errors::testing_prelude::*;
/// #
/// let err = fail_all_vec(
///     vec![Ok(A), Err(ErrA), Ok(A)],
///     |res| res.err().map(HighLevelErr::from).unwrap_or(HighLevelErr::B(ErrB))
/// );
/// assert_eq!(err, Err(vec![ErrB.into(), ErrA.into(), ErrB.into()]));
///
/// let ok = fail_all_vec(vec![Ok(A), Ok(A)], |_: Result<_, ErrA>| ErrC);
/// assert_eq!(ok, Ok(vec![A, A]));
/// ```
pub fn fail_all_vec<T, E1, E2, F>(results: Vec<Result<T, E1>>, f: F) -> Result<Vec<T>, Vec<E2>>
where
    F: FnMut(Result<T, E1>) -> E2,
{
    fail_all(results, f)
        .map(Iterator::collect)
        .map_err(Iterator::collect)
}
