use crate::tcp_udp::TransportTcpUdp;

use super::*;

pub trait SessionRpsObj = GameSessionObj<dyn PlayerRpsObj, 2>;
pub trait SessionServerRpsObj: SessionRpsObj + SessionServerObj
{

}
impl<T> SessionServerRpsObj for T
where
    T: SessionRpsObj + SessionServerObj
{

}
pub trait SessionClientRpsObj: SessionRpsObj + SessionClientObj
{

}
impl<T> SessionClientRpsObj for T
where
    T: SessionRpsObj + SessionClientObj
{

}