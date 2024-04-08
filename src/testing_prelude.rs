//! Placeholders that are used in documentation and tests.

/// A "switch" to control the output of fallible placeholder functions.
#[derive(Debug)]
pub enum Outcome {
    Fail,
    Succeed,
}

pub use Outcome::{Fail, Succeed};

#[derive(Debug, PartialEq, Eq)]
pub struct A;

#[derive(Debug, PartialEq, Eq)]
pub struct ErrA;

pub fn a(outcome: Outcome) -> Result<A, ErrA> {
    match outcome {
        Succeed => Ok(A),
        Fail => Err(ErrA),
    }
}

pub fn action_a() -> Result<A, ErrA> {
    a(Succeed)
}

#[derive(Debug, PartialEq, Eq)]
pub struct B;

#[derive(Debug, PartialEq, Eq)]
pub struct ErrB;

pub fn b(outcome: Outcome) -> Result<B, ErrB> {
    match outcome {
        Succeed => Ok(B),
        Fail => Err(ErrB),
    }
}

pub fn action_b() -> Result<B, ErrB> {
    b(Succeed)
}

#[derive(Debug, PartialEq, Eq)]
pub struct C;

#[derive(Debug, PartialEq, Eq)]
pub struct ErrC;

pub fn c(outcome: Outcome) -> Result<C, ErrC> {
    match outcome {
        Succeed => Ok(C),
        Fail => Err(ErrC),
    }
}

pub fn action_c() -> Result<C, ErrC> {
    c(Succeed)
}

#[derive(Debug, PartialEq, Eq)]
pub enum HighLevelErr {
    A(ErrA),
    B(ErrB),
    C(ErrC),
}

impl From<ErrA> for HighLevelErr {
    fn from(a: ErrA) -> Self {
        HighLevelErr::A(a)
    }
}

impl From<ErrB> for HighLevelErr {
    fn from(b: ErrB) -> Self {
        HighLevelErr::B(b)
    }
}

impl From<ErrC> for HighLevelErr {
    fn from(c: ErrC) -> Self {
        HighLevelErr::C(c)
    }
}
