#![allow(dead_code)]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(unsize)]
#![feature(decl_macro)]
#![feature(negative_impls)]
#![recursion_limit = "256"]
#![feature(trait_upcasting)]
#![feature(coerce_unsized)]
#![feature(inherent_associated_types)]
#![feature(iter_next_chunk)]
#![feature(array_try_map)]
#![feature(auto_traits)]
#![feature(never_type)]
#![feature(associated_type_bounds)]

pub mod game;
pub mod game_rps;
pub mod player;
pub mod error;
#[macro_use]
pub mod castaway;
pub mod boxed;
pub mod traitops;

use std::net::SocketAddr;

use game::*;
use game_rps::*;
use player::*;
use error::*;
use castaway::*;
use boxed::*;

#[cfg(test)]
mod test;

fn main() -> Result<(), ApplicationError>
{
    let target: SocketAddr = SocketAddr::new(local_ip_address::local_ip().unwrap(), 6666);

    let mut server_thread = Some(std::thread::Builder::new()
        .name("Server Main".to_string())
        .spawn(move || main_server(target.port()))?);
    
    let mut client_thread = Some(std::thread::Builder::new()
        .name("Client Main".to_string())
        .spawn(move || main_client(target))?);

    while server_thread.is_some() || client_thread.is_some()
    {
        if let Some(thread) = &mut server_thread
        {
            if thread.is_finished()
            {
                server_thread.take().unwrap().join().map_err(|error| ApplicationError::ThreadError(error))??;
            }
        }
        if let Some(thread) = &mut client_thread
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

    let mut rps_game = GameRps::new(Box::new(session));

    rps_game.game_loop(&mut ui)?;

    Ok(())
}

fn main_client(target: SocketAddr) -> Result<(), ApplicationError>
{
    let mut ui = TUI;
    let mut session = SessionTcpUdp::new_client(target)?;
    session.try_join(&mut ui)?;

    let mut rps_game = GameRps::new(Box::new(session));

    rps_game.game_loop(&mut ui)?;

    Ok(())
}