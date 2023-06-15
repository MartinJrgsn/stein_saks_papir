use crate::{player::PlayerObj, castaway::convert::TryConvert, game::PoisonError};

use super::AddSomePlayerError;

pub enum SpinError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    PoisonError(PoisonError),
    AddSomePlayerError(AddSomePlayerError<PlayerType>)
}
impl<PlayerType> From<PoisonError> for SpinError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn from(error: PoisonError) -> Self
    {
        Self::PoisonError(error)
    }
}
impl<T, PlayerType> From<std::sync::PoisonError<T>> for SpinError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn from(error: std::sync::PoisonError<T>) -> Self
    {
        Self::PoisonError(error.into())
    }
}
impl<T, PlayerType> From<T> for SpinError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized,
    T: Into<AddSomePlayerError<PlayerType>> + ?Sized
{
    fn from(error: T) -> Self
    {
        Self::AddSomePlayerError(error.into())
    }
}