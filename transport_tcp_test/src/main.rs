#![feature(string_remove_matches)]

use std::{net::SocketAddr, sync::{Arc, RwLock}};

use transport::{event::OnConnect, ParaListener};
use transport_tcp::{error::TcpListenerError, Port, TransportTcp};


const PORT: Port = 7878;

fn main()
{
    let target = SocketAddr::new(local_ip_address::local_ip().unwrap(), PORT);

    let transport = Arc::new(RwLock::new(TransportTcp));

    let listener = Arc::new(RwLock::new(ParaListener::<String, String, _>::new("listener test", target, Arc::downgrade(&transport)).unwrap()));
    
    println!("Target: {}", listener.read().unwrap().get_target());
    
    let listener2 = listener.clone();

    std::thread::spawn(move || loop
    {
        if let Some((addr, message)) = listener2.read().unwrap().receive().unwrap()
        {
            println!("Received \"{}\" from {}", message.unwrap(), addr)
        }
    });

    fn update_connections(
        listener: &Arc<RwLock<ParaListener::<String, String, TransportTcp>>>
    )
        -> Result<(), TcpListenerError<SocketAddr>>
    {
        let target = listener.read()
            .unwrap()
            .get_target();

        listener.write()
            .unwrap()
            .check_thread()?;

        let (events, result) = listener.write()
            .unwrap()
            .update_connections();

        for (addr, event) in events
        {
            match event
            {
                OnConnect::NewConnection => {
                    listener.read()
                        .unwrap()
                        .send(addr, format!("You ({}) have successfully connected to {}", addr, target))
                        .unwrap();

                    println!("{} connected", addr)
                },
                OnConnect::DuplicateConnection(stream) => {
                    stream.send(format!("You ({}) are already connected to {}", addr, target))
                        .unwrap();

                    println!("{} attemted to connect when already connected", addr)
                },
                OnConnect::ConnectError(error) => println!("{} attempted to connect with error:\n{:?}", addr, error)
            }
            
        }

        result
    }

    let mut has_connections = None;
    loop
    {
        update_connections(&listener).unwrap();

        let recipient = loop
        {

            let connections = listener.read()
                .unwrap()
                .get_connected_ids();

            let connection_count = connections.len();
            
            if connection_count == 0
            {
                if has_connections != Some(false)
                {
                    println!("Waiting for users to connect...");
                }
                has_connections = Some(false);
                
                update_connections(&listener).unwrap();
            }
            else
            {
                has_connections = Some(true);
                
                println!("Please enter a number within range 0..{} to select message recipient:", connection_count);
                for (i, connection) in connections.clone()
                    .into_iter()
                    .enumerate()
                {
                    println!("{} -> {}", i, connection)
                }
                
                let mut keyboard_input = String::new();
                let size = std::io::stdin()
                    .read_line(&mut keyboard_input)
                    .unwrap();

                keyboard_input.remove_matches(' ');
                keyboard_input.remove_matches('\n');
                keyboard_input.remove_matches('\r');
                keyboard_input.remove_matches('\t');

                if size != 0
                {
                    match keyboard_input.parse::<usize>()
                    {
                        Ok(i) => if let Some(recipient) = connections.get(i)
                        {
                            break *recipient
                        }
                        else
                        {
                            println!("Index \"{}\" is out of bounds.", i)
                        },
                        Err(error) => println!("{:?}\nInput \"{}\" is not a valid index.", error, keyboard_input)
                    }
                }
            }
        };

        let mut keyboard_input = String::new();
        let size = std::io::stdin()
            .read_line(&mut keyboard_input)
            .unwrap();

        if size != 0
        {
            listener.read()
                .unwrap()
                .send(recipient, keyboard_input)
                .unwrap()
        }
    }
}