# `multiple_errors`

Propagate multiple errors instead of just the first one.

## Description

Rust's built-in `?` operator causes an early return on the first encountered
error. However,
[sometimes](https://users.rust-lang.org/t/accumulating-multiple-errors-error-products/93730)
we want to execute multiple independent fallible actions and then report all
errors at once.

This crate covers this use case and aims to become an "umbrella" for more "mass
error handling" fuctionality.

## Example

```rust
use multiple_errors::return_multiple_errors;
use multiple_errors::testing_prelude::*;

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

## Details

Currently, `multiple_errors` is a lightweight "pure logic" crate with no
dependencies, similar to
[itertools](https://github.com/rust-itertools/itertools). It should be (at least
partially) suitable for `no_std` and old MSRVs. Please open an issue or pull
request if you need these features.

## Similar crates

- [frunk::validated::Validated](https://docs.rs/frunk/0.4.2/frunk/validated/enum.Validated.html)

    > A Validated is either an Ok holding an HList or an Err, holding a vector
    > of collected errors.

    It achieves similar goals to `return_multiple_errors!`, but in a more
    abstract, type-heavy and composable way.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
