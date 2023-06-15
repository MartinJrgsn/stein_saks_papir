#![feature(try_trait_v2)]

use std::{ops::Try, convert::Infallible};

pub trait ResultKind: Try<Output = Self::Ok, Residual = Result<Infallible, Self::Err>>
{
    type Ok;
    type Err;
}
impl<Ok, Err> ResultKind for Result<Ok, Err>
{
    type Ok = Ok;
    type Err = Err;
}