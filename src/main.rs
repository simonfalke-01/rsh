use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::env;

struct Config {
    port: i32,
}

fn args() -> Config {
    let args: Vec<String> = env::args().collect();
    let conf = Config {
        port: args.get(1).expect("no port specified").parse().unwrap(),
    };
    conf
}

fn send_message(stream: &mut TcpStream, message: &String) {
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}


fn get_command() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input
}

fn receive(stream: &mut TcpStream) -> String {
    let mut data = String::new();
    loop {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let converted = String::from_utf8_lossy(&buffer).to_string();

        if converted == "MESSAGEDONE" {
            break;
        }

        println!("pushed");
        data.push_str(&converted);
    }

    data
}


fn handle_client(mut stream: TcpStream) {
    loop {
        let command = get_command();
        send_message(&mut stream, &command);
        loop {
            let data = receive(&mut stream);
            if data == "END" {
                break;
            }
            println!("{}", data);
        }
    }
}


fn main() {
    let conf: Config = args();
    let listener = TcpListener::bind(format!("127.0.0.1:{}", conf.port.to_string())).unwrap();
    println!("Listening at 127.0.0.1 on port {}", conf.port);
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