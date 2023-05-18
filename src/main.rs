mod threads;

use std::{net::{TcpListener, TcpStream}, io::{BufRead, BufReader, Write}, thread, time::Duration};
use crate::threads::ThreadPool;

const MAX_CONNECTIONS: usize = 32;
const SEND_DATA_TIMEOUT_SECONDS: u64 = 2;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(MAX_CONNECTIONS);

    for stream in listener.incoming() {
        pool.execute(|| new_connection(stream.unwrap()));
    }

    println!("Shutting down...");
}

fn new_connection(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    match BufReader::new(&mut stream).lines().next() {
        Some(result) => match result {
            Ok(line) => {
                if !line.starts_with("SSH-") {
                    println!("Ignoring non-SSH connection.");
                    return;
                }
            
                println!("Incoming connection from {:#?} with ssh client {}. Sending data to it every {} seconds to keep it busy.", peer_addr, line, SEND_DATA_TIMEOUT_SECONDS);
                while stream.write_all(&"You shall not pass".as_bytes()).is_ok() {
                    thread::sleep(Duration::from_secs(SEND_DATA_TIMEOUT_SECONDS));
                }
            },
            Err(err) => println!("Disconnecting {} ({}).", peer_addr, err),
        },
        None => println!("{} didn't send any data.", peer_addr),
    }

    println!("{} disconnected.", peer_addr);
}