use clap::Parser;
mod client;
mod server;

#[derive(Parser)]
#[command(name = "rsh")]
#[command(author = "Sherlock Holmes, Simonfalke, Canaris")]
#[command(version = "0.0.1")]
#[command(about = "A Rust reverse shell", long_about = None)]

struct Config {
	/// server/client
	mode: String,
	/// Port to operate on.
	#[arg(default_value_t = 8080)]
	port: i32,
	/// IP address to start the server on.
	#[arg(default_value_t = String::from("localhost"))]
	ip: String,
}

fn main() {
	let config = Config::parse();
	if config.mode == "server" {
		server::server(server::Config {
			ip: config.ip,
			port: config.port as u32,
		});
	} else if config.mode == "client" {
		client::client(client::Config {
			ip: config.ip,
			port: config.port as u32,
		});
	} else {
		println!("Invalid mode");
	}
}
