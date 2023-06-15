use super::*;

pub struct PlayersWriteGuard<'a, const PLAYER_COUNT: usize>
{
    actor: &'a ActorServer<PLAYER_COUNT>,
    guard: RwLockWriteGuard<'a, [Option<Box<dyn PlayerObj>>; PLAYER_COUNT]>
}