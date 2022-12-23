use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fmt;


fn handle_client(stream: TcpStream) {

}

fn main() {
    let listener = TcpListener::bind(fmt::format("127.0.0.1:{}", 8080)).unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}