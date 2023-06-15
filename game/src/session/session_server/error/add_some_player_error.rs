use super::*;

pub enum AddSomePlayerError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    AddPlayerError(AddPlayerError<PlayerType>),
    CannotConvert(Box<dyn PlayerObj>)
}
impl<PlayerType> From<AddPlayerError<PlayerType>> for AddSomePlayerError<PlayerType>
where
    PlayerType: PlayerObj + TryConvert<dyn PlayerObj> + ?Sized
{
    fn from(value: AddPlayerError<PlayerType>) -> Self
    {
        Self::AddPlayerError(value)
    }
}