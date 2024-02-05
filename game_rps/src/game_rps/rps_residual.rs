use crate::{Choice, PlayerRpsObj, SessionRps};

pub struct RpsResidual
{
    pub choice_log: Vec<[Choice; 2]>,
    pub session: SessionRps
}