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

extern crate duct;

use clap::Parser;

pub mod client;
pub mod server;

#[derive(Parser)]
#[command(name = "rsh")]
#[command(author = "simonfalke, Sherlock Holmes, Admiral Canaris")]
#[command(version = "0.0.1")]
#[command(about = "A Rust reverse shell", long_about = None)]

pub struct Config {
	///Select the mode to operate on: client or server.
	mode: String,
	/// IP address to start the server on.
	#[arg(default_value_t = String::from("localhost"))]
	ip: String,
	/// Port to operate on.
	#[arg(default_value_t = 8080)]
	port: i32,
}

fn main() {
	let conf: Config = Config::parse();

	match conf.mode.as_str() {
		"server" => {
			server::server::main(conf);
		}

		"client" => {
			client::client::main(conf);
		}

		other => {
			println!("{} is not a valid mode.", other);
		}
	}
}
