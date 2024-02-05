use std::net::SocketAddr;

use game::{message::{ClientMessage, ServerMessage}, session::Session};
use transport_tcp::{error::TcpMessageError, TransportTcp};

use crate::{message::{RpsClientMessageData, RpsServerMessageData}, RpsEndState};

pub type SessionRps = Session<RpsClientMessageData, RpsServerMessageData, SocketAddr, RpsEndState, TransportTcp>;