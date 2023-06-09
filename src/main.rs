#![allow(dead_code)]

pub mod game;
pub mod game_rps;
pub mod player;
pub mod error;

use std::net::SocketAddr;

use game::*;
use game_rps::*;
use player::*;
use error::*;

#[cfg(test)]
mod test;

fn main() -> Result<(), ApplicationError>
{
    const TARGET: SocketAddr = SocketAddr::new(local_ip_address::local_ip().unwrap(), 6666);

    let server_thread = Some(std::thread::Builder::new()
        .name("Server Main".to_string())
        .spawn(|| main_server(TARGET.port()))?);
    
    let client_thread = Some(std::thread::Builder::new()
        .name("Client Main".to_string())
        .spawn(|| main_client(TARGET))?);

    while server_thread.is_some() || client_thread.is_some()
    {
        if let Some(thread) = server_thread
        {
            if thread.is_finished()
            {
                server_thread.take().unwrap().join().map_err(|error| ApplicationError::ThreadError(error))??;
            }
        }
        if let Some(thread) = client_thread
        {
            if thread.is_finished()
            {
                client_thread.take().unwrap().join().map_err(|error| ApplicationError::ThreadError(error))??;
            }
        }
    }
    
    Ok(())
}

fn main_server(port: u16) -> Result<(), ApplicationError>
{
    let mut ui = TUI;
    let mut session = SessionTcpUdp::new_host(port)?;
    session.try_join(&mut ui)?;

    let mut rps_game = SteinSaksPapir::new(Box::new(session));

    rps_game.game_loop();

    Ok(())
}

fn main_client(target: SocketAddr) -> Result<(), ApplicationError>
{
    let mut ui = TUI;
    let mut session = SessionTcpUdp::new_client(target)?;
    session.try_join(&mut ui)?;

    let mut rps_game = SteinSaksPapir::new(Box::new(session));

    rps_game.game_loop();

    Ok(())
}