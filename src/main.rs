mod commands;
mod inmem;
mod parser;
mod pool;

pub use commands::Command;
pub use parser::parse;

use inmem::InMem;
use pool::ThreadPool;

use std::{
    env,
    io::{Read, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    str,
    sync::{Arc, Mutex},
};

fn handle_client(stream: &mut TcpStream, inmem: Arc<Mutex<InMem>>) {
    let conn = stream.peer_addr().unwrap().to_string();
    println!("Connection from {}", conn);

    let mut buf = [0u8; 1024];
    while let Ok(bytes) = stream.read(&mut buf) {
        if bytes < 1 {
            break;
        }

        let command = str::from_utf8(&buf[..bytes]).unwrap();

        let message = match inmem.lock().unwrap().execute(command) {
            Ok(message) => format!("OK: {}\n", message),
            Err(message) => format!("ERR: {}\n", message),
        };

        stream.write(message.as_bytes()).ok();
    }
    println!("Connection {} closed", conn);
}

fn main() -> std::io::Result<()> {
    let port = env::args()
        .nth(1)
        .expect("Port not provided")
        .parse::<u16>()
        .expect("Incorrect port");

    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port))?;
    println!("Listening on {}", port);

    let pool = ThreadPool::new(12);

    let inmem = Arc::new(Mutex::new(InMem::new()));

    for stream in listener.incoming() {
        let arc = Arc::clone(&inmem);
        pool.execute(move || handle_client(&mut stream.unwrap(), arc));
    }

    Ok(())
}
