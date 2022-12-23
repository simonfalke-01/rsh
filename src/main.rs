use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::fmt;


fn send_message(stream: &TcpStream, message: &str) {
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn get_command() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}


fn receive(stream: &TcpStream) -> String {
    let mut buffer = [0; 512];
    let mut data = String::new();
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        data.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
    }
}


fn handle_client(stream: TcpStream) {
    loop {
        let command = get_command();
        send_message(&stream, &command);
        loop {
            let data = receive(&stream);
            if data == "END" {
                break;
            }
            println!("{}", data);
        }
    }
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