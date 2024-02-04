use std::{marker::Tuple, ops::Try, time::{Duration, SystemTime}};

use option_trait::{Optional, Maybe};

use super::*;

pub trait RepeatUntil<Args>: Fn<Args>
where
    Args: Tuple + Copy + ?Sized
{
    fn repeat_until(&self, criteria: impl Fn(Self::Output) -> bool, args: Args, timeout: Duration) -> Result<(), RepeatError>
    {
        let begin_time = SystemTime::now();
        while !criteria(self.call(args))
        {
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::from(TimeoutError(timeout)))
            }
        }
        Ok(())
    }
}
impl<F, Args> RepeatUntil<Args> for F
where
    F: Fn<Args> + ?Sized,
    Args: Tuple + Copy + ?Sized {}

pub trait RepeatUntilSome<Args>: Fn<Args, Output: Optional> + RepeatUntil<Args>
where
    Args: Tuple + Copy + ?Sized
{
    fn repeat_until_some(&self, args: Args, timeout: Duration) -> Result<<Self::Output as Optional>::Some, RepeatError>
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
                return Err(RepeatError::from(TimeoutError(timeout)))
            }
        }
    }
}
impl<F, Args> RepeatUntilSome<Args> for F
where
    F: Fn<Args, Output: Optional> + ?Sized,
    Args: Tuple + Copy + ?Sized {}

pub trait RepeatUntilBool<Args>: Fn<Args, Output = bool> + RepeatUntil<Args>
where
    Args: Tuple + Copy + ?Sized
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
    F: Fn<Args, Output = bool> + ?Sized,
    Args: Tuple + Copy + ?Sized {}

// TRY

pub trait TryRepeatUntil<Args>: Fn<Args, Output: Try + Into<Result<<<Self as FnOnce<Args>>::Output as Try>::Output, <<Self as FnOnce<Args>>::Output as Try>::Residual>>>
where
    Args: Tuple + Copy + ?Sized
{
    fn try_repeat_until(
        &self,
        criteria: impl Fn(<<Self as FnOnce<Args>>::Output as Try>::Output) -> bool,
        args: Args,
        timeout: Duration
    ) -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as Try>::Residual>>
    {
        let begin_time = SystemTime::now();
        while !criteria(self.call(args).into().map_err(|error| TryRepeatError::FnError(error))?)
        {
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::from(TimeoutError(timeout)).into())
            }
        }
        Ok(())
    }
}
impl<F, Args> TryRepeatUntil<Args> for F
where
    F: Fn<Args, Output: Try + Into<Result<<<Self as FnOnce<Args>>::Output as Try>::Output, <<Self as FnOnce<Args>>::Output as Try>::Residual>>> + ?Sized,
    Args: Tuple + Copy + ?Sized {}

pub trait TryRepeatUntilSome<Args>: Fn<Args, Output: Try<Output: Optional> + Into<Result<<<Self as FnOnce<Args>>::Output as Try>::Output, <<Self as FnOnce<Args>>::Output as Try>::Residual>>>
where
    Args: Tuple + Copy + ?Sized
{
    fn try_repeat_until_some(&self, args: Args, timeout: Duration)
        -> Result<
            <<<Self as FnOnce<Args>>::Output as Try>::Output as Optional>::Some,
            TryRepeatError<<<Self as FnOnce<Args>>::Output as Try>::Residual>
        >
    {
        let begin_time = SystemTime::now();
        loop
        {
            if let Some(value) = self.call(args)
                .into()
                .map_err(|error| TryRepeatError::FnError(error))?
                .into_option()
            {
                return Ok(value)
            }
            if begin_time.elapsed()? >= timeout
            {
                return Err(RepeatError::from(TimeoutError(timeout)).into())
            }
        }
    }
}
impl<F, Args, T> TryRepeatUntilSome<Args> for F
where
    F: Fn<Args, Output: Try<Output = Option<T>> + Into<Result<<<Self as FnOnce<Args>>::Output as Try>::Output, <<Self as FnOnce<Args>>::Output as Try>::Residual>>> + ?Sized,
    Args: Tuple + Copy + ?Sized {}

pub trait TryRepeatUntilBool<Args>: Fn<Args, Output: Try<Output = bool>> + TryRepeatUntil<Args>
where
    Args: Tuple + Copy + ?Sized
{
    fn try_repeat_until_true(&self, args: Args, timeout: Duration)
        -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as Try>::Residual>>
    {
        self.try_repeat_until(|value| value == true, args, timeout)
    }
    fn try_repeat_until_false(&self, args: Args, timeout: Duration)
        -> Result<(), TryRepeatError<<<Self as FnOnce<Args>>::Output as Try>::Residual>>
    {
        self.try_repeat_until(|value| value == false, args, timeout)
    }
}
impl<F, Args> TryRepeatUntilBool<Args> for F
where
    F: Fn<Args, Output: Try<Output = bool> + Into<Result<bool, <<Self as FnOnce<Args>>::Output as Try>::Residual>>> + ?Sized,
    Args: Tuple + Copy + ?Sized {}