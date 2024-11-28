use std::{io::{BufReader, BufWriter, Read}, net::{TcpListener, TcpStream}, process::exit, sync::{Arc, Mutex}};

use crate::battle::Battle;



fn handle_client(mut stream: TcpStream, battle: Arc<Mutex<Battle>>, clients: Arc<Mutex<Vec<TcpStream>>>) {
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        };

        let msg = String::from_utf8_lossy(&buffer[..bytes_read]);
        let mut battle_guard = battle.lock().unwrap();


        

    }
}