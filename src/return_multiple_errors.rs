/// Given a bunch of `Result`s, return a non-empty `Vec` of errors or unwrap
/// all `Result`s and proceed.
///
/// ## Usage
///
/// The first statement should define a mutable `Vec` of errors, usually empty.
///
/// The statements that follow should define `Result` variables with the same
/// (or convertible) error type.
///
/// Finally, the last statement should define a diverging branch with the
/// following shape:
///
/// ```ignore
/// if_there_are_errors {
///   return /* the vec of errors or some other expression */;
/// }
/// ```
///
/// ## Examples
///
/// ```no_run
/// # use multiple_errors::return_multiple_errors;
/// # use multiple_errors::testing_prelude::*;
/// #
/// fn a_b_c() -> Result<(A, B, C), Vec<HighLevelErr>> {
///     return_multiple_errors!(
///         // Annotate the "common" error type for the container of errors.
///         let mut errors: Vec<HighLevelErr> = vec![];
///         // Get some `Result`s.
///         let a = action_a();
///         let b = action_b();
///         let c = action_c();
///         // If there are any errors, they are implicitly converted and collected.
///         if_there_are_errors {
///             return Err(errors);
///         }
///     );
///     // And here we can proceed on the happy path, with already-unwrapped `Ok` values.
///     // `a`, `b`, and `c` are no longer `Result`s. They have been shadowed.
///     Ok((a, b, c))
/// }
/// ```
///
/// ## Some nice details
///
/// - `Ok` types don't have to be the same
/// - Errors don't have to be `Clone`
#[macro_export]
macro_rules! return_multiple_errors {
    (
        let mut $errors:ident: Vec<$E:ty> = $initial_errors:expr;
        $(
           let $var:ident = $expr:expr;
        )+
        if_there_are_errors {
           return $return_val:expr;
        }
    ) => {
        $(
            let $var = $expr;
        )+
        let ( $( $var, )+ ) = match ( $( $var, )+ ) {
            ( $( Ok($var), )+ ) => ( $( $var, )+ ),
            ( $( $var, )+ ) => {
                let mut $errors: Vec<$E> = $initial_errors;
                $(
                    if let Err(err) = $var {
                        $errors.push(err.into());
                    }
                )+
                return $return_val;
            }
        };
    };
}

#[cfg(test)]
mod tests {
    use crate::testing_prelude::*;

    fn a_b(outcome_a: Outcome, outcome_b: Outcome) -> Result<(A, B), Vec<HighLevelErr>> {
        return_multiple_errors!(
            let mut errors: Vec<HighLevelErr> = vec![];
            let a = a(outcome_a);
            let b = b(outcome_b);
            if_there_are_errors {
                return Err(errors);
            }
        );
        Ok((a, b))
    }

    #[test]
    fn both_errors() {
        assert_eq!(a_b(Fail, Fail), Err(vec![ErrA.into(), ErrB.into()]))
    }

    #[test]
    fn either_error() {
        assert_eq!(a_b(Fail, Succeed), Err(vec![ErrA.into()]));
        assert_eq!(a_b(Succeed, Fail), Err(vec![ErrB.into()]))
    }

    #[test]
    fn no_errors() {
        assert!(a_b(Succeed, Succeed).is_ok())
    }
}
