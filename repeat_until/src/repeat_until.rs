use std::{time::{SystemTime, Duration}, marker::Tuple, ops::FromResidual};

use crate::{option_kind::OptionKind, result_kind::ResultKind};

use super::*;

pub trait RepeatUntil<Args>: Fn<Args>
where
    Args: Tuple
{
    fn repeat_until(&self, criteria: impl Fn(Self::Output) -> bool, args: Args, timeout: Duration) -> Result<(), RepeatError>
    {
        let begin_time = SystemTime::now();
        while !criteria(self.call(args))
        {
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::Timeout(timeout))
            }
        }
        Ok(())
    }
}
impl<F, Args> RepeatUntil<Args> for F
where
    F: Fn<Args>,
    Args: Tuple {}

pub trait RepeatUntilSome<Args>: Fn<Args, Output: OptionKind> + RepeatUntil<Args>
where
    Args: Tuple
{
    fn repeat_until_some(&self, args: Args, timeout: Duration) -> Result<<Self::Output as OptionKind>::Some, RepeatError>
    {
        let begin_time = SystemTime::now();
        loop
        {
            if let Some(value) = self.call(args).into_option()
            {
                return Ok(value)
            }
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::Timeout(timeout))
            }
        }
    }
}
impl<F, Args> RepeatUntilSome<Args> for F
where
    F: Fn<Args, Output: OptionKind>,
    Args: Tuple {}

pub trait RepeatUntilBool<Args>: Fn<Args, Output = bool> + RepeatUntil<Args>
where
    Args: Tuple
{
    fn repeat_until_true(&self, args: Args, timeout: Duration) -> Result<(), RepeatError>
    {
        self.repeat_until(|value| value == true, args, timeout)
    }
    fn repeat_until_false(&self, args: Args, timeout: Duration) -> Result<(), RepeatError>
    {
        self.repeat_until(|value| value == false, args, timeout)
    }
}
impl<F, Args> RepeatUntilBool<Args> for F
where
    F: Fn<Args, Output = bool>,
    Args: Tuple {}

// TRY

pub trait TryRepeatUntil<Args>: Fn<Args, Output: ResultKind>
where
    Args: Tuple,
    Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>
{
    fn try_repeat_until(
        &self,
        criteria: impl Fn(<<Self as FnOnce<Args>>::Output as ResultKind>::Ok) -> bool,
        args: Args,
        timeout: Duration
    ) -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Ok>>
    {
        let begin_time = SystemTime::now();
        while !criteria(self.call(args)?)
        {
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::Timeout(timeout).into())
            }
        }
        Ok(())
    }
}
impl<F, Args> TryRepeatUntil<Args> for F
where
    F: Fn<Args, Output: ResultKind>,
    Args: Tuple,
    Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err> {}

pub trait TryRepeatUntilSome<Args>: Fn<Args, Output: ResultKind<Ok: OptionKind>>
where
    Args: Tuple,
    Result<<<<Self as FnOnce<Args>>::Output as ResultKind>::Ok as OptionKind>::Some, RepeatError>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>
{
    fn repeat_until_some(&self, args: Args, timeout: Duration)
        -> Result<<<<Self as FnOnce<Args>>::Output as ResultKind>::Ok as OptionKind>::Some, RepeatError>
    {
        let begin_time = SystemTime::now();
        loop
        {
            if let Some(value) = self.call(args)?.into_option()
            {
                return Ok(value)
            }
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::Timeout(timeout))
            }
        }
    }
}
impl<F, Args> TryRepeatUntilSome<Args> for F
where
    F: Fn<Args, Output: ResultKind<Ok: OptionKind>>,
    Args: Tuple,
    Result<<<<Self as FnOnce<Args>>::Output as ResultKind>::Ok as OptionKind>::Some, RepeatError>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err> {}

pub trait TryRepeatUntilBool<Args>: Fn<Args, Output: ResultKind<Ok = bool>> + TryRepeatUntil<Args>
where
    Args: Tuple,
    Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>
{
    fn repeat_until_true(&self, args: Args, timeout: Duration)
        -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>
    {
        self.try_repeat_until(|value| value == true, args, timeout)
    }
    fn repeat_until_false(&self, args: Args, timeout: Duration)
        -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>
    {
        self.try_repeat_until(|value| value == false, args, timeout)
    }
}
impl<F, Args> TryRepeatUntilBool<Args> for F
where
    F: Fn<Args, Output: ResultKind<Ok = bool>>,
    Args: Tuple,
    Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as ResultKind>::Err>>:
        FromResidual<<<Self as FnOnce<Args>>::Output as ResultKind>::Err> {}