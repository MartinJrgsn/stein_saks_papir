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
    use std::{net::{TcpListener, TcpStream}, io::{Write, Read}};
    const PORT: &str = "6666";
    //const TARGET: &str = "192.168.2.2";
    const MARTIN: &str = "192.168.43.20";
    const SIGURD: &str = "192.168.43.147";
    const DESTINATION: &str = MARTIN;
    const SOURCE: &str = SIGURD;
    //const TARGET: &str = "192.168.43.20";
    
    #[test]
    fn test_tcp_ping_server()
    {
        let listener = loop
        {
            match TcpListener::bind(DESTINATION.to_string() + ":" + PORT)
            {
                Ok(listener) => break listener,
                Err(err) => ()
            }
        };
        for stream in listener.incoming()
        {
            let mut stream = stream.expect("Invalid stream!");
    
            let mut request_message = vec![];
            stream.read(&mut request_message).expect("Unable to read");
            stream.write_all(&[b"Ping test! all good! Received: {}".to_vec(), request_message].concat())
                .expect("Unable to write");
            
            println!("Connection established!");
        }
    }
    
    #[test]
    fn test_tcp_ping_client()
    {    
        let mut stream = TcpStream::connect(DESTINATION.to_string() + ":" + PORT)
            .expect("Unable to connect!");
        stream.write(b"Ping!").expect("Unable to write ping");
    
        let listener = TcpListener::bind(SOURCE.to_string() + ":" + PORT).expect("Unable to bind!");
        for stream in listener.incoming()
        {
            let mut stream = stream.expect("Invalid stream!");
    
            let mut request_message = vec![];
            stream.read(&mut request_message).expect("Unable to read");
            
            println!("Connection established! Received: {}", String::from_utf8(request_message).expect("Invalid string"));
        }
    }
}