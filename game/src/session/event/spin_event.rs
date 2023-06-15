use super::*;

pub enum SpinEvent
{
    OnJoin(OnJoinEvent),
    OnSpawnListener,
    OnRespawnListener
}