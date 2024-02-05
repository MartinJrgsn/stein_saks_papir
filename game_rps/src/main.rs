#![allow(dead_code)]
#![feature(specialization)]
#![feature(trait_alias)]
#![feature(unsize)]
#![feature(decl_macro)]
#![recursion_limit = "256"]
#![feature(trait_upcasting)]
#![feature(coerce_unsized)]
#![feature(inherent_associated_types)]
#![feature(iter_next_chunk)]
#![feature(array_try_map)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![feature(never_type)]
#![feature(associated_type_bounds)]
#![feature(unboxed_closures)]
#![feature(try_trait_v2)]
#![feature(tuple_trait)]
#![feature(associated_const_equality)]
#![feature(fn_traits)]
#![feature(split_array)]
#![feature(variant_count)]

moddef::moddef!(
    flat(pub) mod {
        game_rps,
        ui_rps,
        player_rps,
        session_rps
    },
    pub mod {
        message,
        error
    }
);

use std::{net::SocketAddr, sync::{Arc, RwLock, Weak}, time::Duration};

use error::ApplicationError;
use game::{actor::{Actor, ActorKind}, game::Game};
use transport::{error::{JoinThreadError, SpawnThreadError}, ParaListener, ParaStream};
use transport_tcp::TransportTcp;

fn main() -> Result<(), ApplicationError>
{
    const TIMEOUT: Duration = Duration::new(10, 0);
    let target: SocketAddr = SocketAddr::new(local_ip_address::local_ip().unwrap(), 6666);
    let transport = Arc::new(RwLock::new(TransportTcp));
    let transport_weak = Arc::downgrade(&transport);

    let mut server_thread = Some(std::thread::Builder::new()
        .name("Server Main".to_string())
        .spawn(move || main_server(target, transport_weak))
        .map_err(|error| SpawnThreadError(error))?);
    
    let transport_weak = Arc::downgrade(&transport);
    
    let mut client_thread = Some(std::thread::Builder::new()
        .name("Client Main".to_string())
        .spawn(move || main_client(target, transport_weak))
        .map_err(|error| SpawnThreadError(error))?);

    while server_thread.is_some() || client_thread.is_some()
    {
        if let Some(thread) = &mut server_thread
        {
            if thread.is_finished()
            {
                server_thread.take()
                    .unwrap()
                    .join()
                    .map_err(|error| JoinThreadError(error))??;
            }
        }
        if let Some(thread) = &mut client_thread
        {
            if thread.is_finished()
            {
                client_thread.take()
                    .unwrap()
                    .join()
                    .map_err(|error| JoinThreadError(error))??;
            }
        }
    }
    
    Ok(())
}

fn main_server(target: SocketAddr, transport: Weak<RwLock<TransportTcp>>) -> Result<(), ApplicationError>
{
    let mut ui = TUI;
    let session = SessionRps::new(2, ActorKind::Server, "RPS Server", target, transport)?;

    let rps_game = GameRps::new(session);

    let (result, _) = rps_game.game_loop(&mut ui);
    println!("Game End: {}", result?);

    Ok(())
}

fn main_client(target: SocketAddr, transport: Weak<RwLock<TransportTcp>>) -> Result<(), ApplicationError>
{
    let mut ui = TUI;
    let session = SessionRps::new(2, ActorKind::Client, "RPS Client", target, transport)?;

    let rps_game = GameRps::new(session);

    let (result, _) = rps_game.game_loop(&mut ui);
    println!("Game End: {}", result?);

    Ok(())
}