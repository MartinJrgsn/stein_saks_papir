use crate::game::*;

#[test]
fn test_win_conditions() -> (){
    fn conditions_to_string(wld : Option<bool>) -> &'static str {
        match wld {
            Some(true)  =>  "Win :-)",
            None        =>  "Draw :-|",
            Some(false) =>  "Loss :-(",
        }
    }

    static TRUTH_TABLE : [[Option<bool>; Choice::LENGTH]; Choice::LENGTH] = [
        [None,          Some(true),     Some(false)],
        [Some(false),   None,           Some(true)],
        [Some(true),    Some(false),    None],
    ];
    
    for i in 0..TRUTH_TABLE.len() {
        for j in 0..TRUTH_TABLE.len() {
            let condition_string = conditions_to_string(TRUTH_TABLE[i][j]);

            match (Choice::try_from(i as u8), Choice::try_from(j as u8)) {
                (Ok(me), Ok(you)) => {
                    let game_outcome = me.get_outcome(you);
                    println!("i: {0}, j: {1}, val: {2}, outcome: {3} ",
                        i,
                        j,
                        condition_string,
                        conditions_to_string(game_outcome),
                    );
                    
                    assert_eq!(game_outcome, TRUTH_TABLE[i][j]);
                }
                _ => panic!("hjølp"),
            }
        }
    }
}

pub mod test_tcp
{
    use std::{net::{TcpListener, TcpStream}, io::{Write, Read, BufReader, BufRead}};
    const PORT: &str = "7878";
    const MARTIN: &str = "192.168.43.20";
    const SIGURD: &str = "172.16.216.132";
    const SERVER: &str = MARTIN;
    const SERVER_IP: &str = "192.168.43.20:7878";
    
    #[test]
    fn test_tcp_message_from_user_server()
    {
        use std::{
            net::{TcpListener, TcpStream},  
            io::{Write, Error, prelude::*, BufReader},
            thread,
        };
        let listener=  TcpListener::bind(SERVER_IP.to_string()).expect("Could not bind!");
    
        for stream in listener.incoming() {
            match stream {
                Err(e) => { eprintln!("failed: {}", e) }
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            }
        }

        fn handle_client(mut stream: TcpStream) -> Result<(), Error> {
            println!("Incloming connection from: {}", stream.peer_addr()?);
            let mut buffer = vec![];
            loop {
                let mut reader = BufReader::new(&stream);
                reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");

                print!("{}", String::from_utf8(buffer.clone()).expect("Failed to parse buffer"));
                stream.write(&buffer)?;
            }
        }
    }
    
    #[test]
    fn test_tcp_message_from_user_client()
    {    
        use std::{
            net::{TcpStream}, 
            io::{self, BufRead, BufReader, Write},
            str
        };

        // const SERVER_IP: &str = "192.168.43.20:7878";

        let mut stream = TcpStream::connect(SERVER_IP.to_string()).expect("Could not connect to server!");
        loop {
            let mut input = String::new();
            let mut buffer : Vec<u8> = Vec::new();
            io::stdin().read_line(&mut input).expect("Failed to read from stdin");
            stream.write(input.as_bytes()).expect("Failed to write to server");

            let mut reader = BufReader::new(&stream);

            reader.read_until(b'\n', &mut buffer).expect("Could not read into buffer");
            print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
        }
    }

    #[test]
    fn test_tcp_ping_server()
    {
        let listener = TcpListener::bind(SERVER.to_string() + ":" + PORT).expect("Unable to bind");
        for stream in listener.incoming()
        {
            let mut stream = stream.expect("Invalid stream!");
    
            let mut request_message = vec![];
            stream.read(&mut request_message).expect("Unable to read");
            stream.write_all(&[b"Ping test all good! Received: ".to_vec(), request_message].concat())
                .expect("Unable to write");
            
            println!("Connection established!");
        }
    }
    
    #[test]
    fn test_tcp_ping_client()
    {
        let mut stream = TcpStream::connect(SERVER.to_string() + ":" + PORT) // Connect to source
            .expect("Unable to connect!");
        stream.write(b"Ping!").expect("Unable to write ping");
        loop
        {
            let mut buffer = vec![];
            let mut reader = BufReader::new(&stream);
            reader.read_until(b'\n', &mut buffer).expect("Unable to read");

            if buffer.len() > 0
            {
                println!("Connection established! Received: {}", String::from_utf8(buffer).expect("Invalid string"));
                break
            }
        }
    }
}