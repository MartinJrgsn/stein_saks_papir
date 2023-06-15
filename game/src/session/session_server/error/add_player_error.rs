use crate::{player::PlayerObj, castaway::convert::TryConvert};

pub enum AddPlayerError<PlayerType>
where
    PlayerType: PlayerObj + ?Sized
{
    AlreadyJoined(Box<PlayerType>),
    GameFull(Box<PlayerType>)
}