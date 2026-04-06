use std::env;
use std::io;

#[derive(Debug, Clone)]
pub struct ServerConfig {
	pub bind_host: String,
	pub bind_port: u16,
}

pub fn load() -> Result<ServerConfig, io::Error> {
	let bind_host = env::var("PE_API_BIND_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
	let raw_port = env::var("PE_API_BIND_PORT").ok();
	let bind_port = match raw_port.as_deref() {
		None => 8080,
		Some(value) => value
			.parse()
			.map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid PE_API_BIND_PORT"))?,
	};
	Ok(ServerConfig { bind_host, bind_port })
}
