#![feature(try_trait_v2)]

use std::{ops::Try, convert::Infallible};

pub trait ResultKind: Try<Output = Self::Ok, Residual = Result<Infallible, Self::Err>>
{
    type Ok;
    type Err;

    fn into_result(self) -> Result<Self::Ok, Self::Err>;
}
impl<Ok, Err> ResultKind for Result<Ok, Err>
{
    type Ok = Ok;
    type Err = Err;

    fn into_result(self) -> Result<<Self as ResultKind>::Ok, <Self as ResultKind>::Err>
    {
        self
    }
}