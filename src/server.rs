use clap::Parser;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

pub struct Config {
	pub ip: String,
	pub port: u32,
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

	let mut buffer = [0; 512];
	stream.read(&mut buffer).unwrap();
	let mut converted = String::from_utf8_lossy(&buffer).to_string();
	converted = converted.replace("\0", "");

	// println!("{}", "pushed".green());
	data.push_str(&converted.trim());

	data
}

fn handle_client(mut stream: TcpStream) {
	loop {
		let command = get_command();
		send_message(&mut stream, &command);
		let mut data = receive(&mut stream);
		data = data.replace("\0", "");
		println!("{}", data);
	}
}

pub fn server(conf: Config) {
	let listener = TcpListener::bind(format!("{}:{}", conf.ip, conf.port.to_string())).unwrap();
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
