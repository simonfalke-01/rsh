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

use std::env;

use rsh::client::run_client;
use rsh::server::run_server;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("Usage: rsh [server | client]");
		return;
	}

	let command = args[1].clone();

	match command.as_str() {
		"server" => server::run_server(),
		"client" => client::run_client(),
		_ => println!("Invalid command. Usage: rsh [server | client]"),
	}
}
