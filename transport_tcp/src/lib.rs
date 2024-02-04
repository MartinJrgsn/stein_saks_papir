#![feature(associated_type_bounds)]
#![feature(is_some_and)]

moddef::moddef!(
    flat(pub) mod {
        transport_tcp
    },
    pub flat mod {
        error
    }
);

pub type Port = u16;

#[cfg(test)]
mod test
{
    use std::{net::{SocketAddr, Ipv4Addr, IpAddr}, sync::{Arc, RwLock}, time::Duration};

    use transport::{ParaListener, ParaStream, event::OnConnect};

    use crate::{Port, TransportTcp, error::TcpListenerError};
    
    const PORT: Port = 7878;

    #[test]
    fn test_listener_chat()
    {
        let target = SocketAddr::new(local_ip_address::local_ip().unwrap(), PORT);

        let transport = Arc::new(RwLock::new(TransportTcp));

        let listener = Arc::new(RwLock::new(ParaListener::<String, String, _>::new("listener test", target, Arc::downgrade(&transport)).unwrap()));
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

    #[test]
    fn test_stream_chat()
    {
        #[allow(unused)]
        const MARTIN: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 168, 43, 20));
        #[allow(unused)]
        const SIGURD: IpAddr = IpAddr::V4(Ipv4Addr::new(172, 16, 216, 132));

        const TARGET_IP: IpAddr = MARTIN;

        const TARGET: SocketAddr = SocketAddr::new(TARGET_IP, PORT);

        let transport = Arc::new(RwLock::new(TransportTcp));

        println!("Connecting to {}...", TARGET);
        let mut prev_error = None;
        let stream = loop
        {
            match ParaStream::<String, String, _>::new("stream test", TARGET, Arc::downgrade(&transport))
            {
                Ok(stream) => break Arc::new(stream),
                Err(error) => {
                    let error_string = format!("{:?}", error);
                    if !prev_error.as_deref().is_some_and(|prev_error| prev_error == error_string)
                    {
                        println!("{}", error_string);
                        prev_error = Some(error_string);
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(1))
        };
        let stream2 = stream.clone();

        std::thread::spawn(move || loop
        {
            if let Some(message) = stream2.receive().unwrap()
            {
                println!("Received: \"{}\"", message.unwrap())
            }
        });

        loop
        {
            let mut keyboard_input = String::new();
            let size = std::io::stdin().read_line(&mut keyboard_input).expect("Failed to read from stdin");

            if size != 0
            {
                stream.send(keyboard_input).unwrap()
            }
        }
    }
}