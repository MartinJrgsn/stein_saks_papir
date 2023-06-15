use super::*;

pub struct PlayersReadGuard<'a, const PLAYER_COUNT: usize>
{
    actor: &'a ActorServer<PLAYER_COUNT>,
    guard: RwLockReadGuard<'a, [Option<Box<dyn PlayerObj>>; PLAYER_COUNT]>
}
impl<'a, const PLAYER_COUNT: usize> PlayersReadGuard<'a, PLAYER_COUNT>
{
    
}