# `multiple_errors`

Propagate multiple errors instead of just the first one.

## Description

Rust's `?` operator and `Iterator::collect::<Result<_, _>>` return early on the
first encountered error. However,
[sometimes](https://users.rust-lang.org/t/accumulating-multiple-errors-error-products/93730)
we want to execute multiple independent actions and then report all errors at
once. Or turn all results into errors if there is at least one error.

This crate covers these use cases and aims to become an easily googlable "hub" for:

- more "mass error handling" fuctionality
- knowledge about related functionality in other crates

Think of `multiple_errors` as
[itertools](https://github.com/rust-itertools/itertools). It's also a
lightweight "pure logic" crate with no dependencies, and should be suitable for
`no_std` and old MSRVs. I haven't worked on this yet, please open an issue or a
pull request if you need this.

## Example

```rust
use multiple_errors::{fail_all_vec, return_multiple_errors, CollectVecResult};
use multiple_errors::testing_prelude::*;

assert_eq!(
    [Err(ErrA), Ok(A), Err(ErrA)].into_iter().collect_vec_result(),
    // Collected all errors, not just the first one
    Err(vec![ErrA, ErrA])
);

let err = fail_all_vec(
    vec![Ok(A), Err(ErrA), Ok(A)],
    |res| res.err().map(HighLevelErr::from).unwrap_or(HighLevelErr::B(ErrB))
);
// Same length as the original, each element turned into an error.
assert_eq!(err, Err(vec![ErrB.into(), ErrA.into(), ErrB.into()]));

fn a_b_c() -> Result<(A, B, C), Vec<HighLevelErr>> {
    return_multiple_errors!(
        let mut errors: Vec<HighLevelErr> = vec![];
        // Get some `Result`s:
        let a = action_a();
        let b = action_b();
        let c = action_c();
        if_there_are_errors {
            // Already converted and collected
            return Err(errors);
        }
    );
    // Already unwrapped
    Ok((a, b, c))
}
```

## Similar crates

- [lazy_errors](https://docs.rs/lazy_errors/0.7.0/lazy_errors/index.html)
  
  > Effortlessly create, group, and nest arbitrary errors, and defer error handling ergonomically.
  
  This crate is `return_multiple_errors!` done right! It appeared a few months
  later than mine. If it had existed, I would have probably just used it.

- [frunk::validated::Validated](https://docs.rs/frunk/0.4.2/frunk/validated/enum.Validated.html)

    > A Validated is either an Ok holding an HList or an Err, holding a vector
    > of collected errors.

    This is somewhat similar to
    [lazy_errors](https://docs.rs/lazy_errors/0.7.0/lazy_errors/index.html) and
    `return_multiple_errors!`, but seems more abstract and type-heavy, the docs
    are harder to follow.

- [itertools::Itertools::partition_result](https://docs.rs/itertools/0.12.1/itertools/trait.Itertools.html#method.partition_result)
    and more general
    [partition_map](https://docs.rs/itertools/0.12.1/itertools/trait.Itertools.html#method.partition_map)

    > Partition a sequence of Results into one list of all the Ok elements and
    > another list of all the Err elements.

    If you need both lists, just use
    [itertools](https://github.com/rust-itertools/itertools). If you discard
    `Ok`s in case of errors, you can use
    `CollectVecResult::collect_vec_result()` that returns
    `Result<Vec<T>, Vec<E>>`. It's more precise and efficient.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
