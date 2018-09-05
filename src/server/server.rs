
use server::{
	player::{Player, PlayerManager, DefaultPlayerManager},
	ZirconServer, ZirconError
};

use std::{
	net::UdpSocket,
	vec::Vec
};

pub struct DefaultServer {
	socket: Option<UdpSocket>,
	player_manager: Box<PlayerManager>
}

impl DefaultServer {

	pub fn new(max_players: u32) -> Self {
		DefaultServer {
			socket: None,
			player_manager: Box::new(DefaultPlayerManager::new(127))
		}
	}
}

impl ZirconServer for DefaultServer {

	fn start(&mut self, address: &str, port: i16) -> Result<(), ZirconError> {
		match UdpSocket::bind(&format!("{}:{}", address, port)) {
			Ok(socket) => {
				self.socket = Some(socket);
				Ok(())
			},
			Err(e) => {
				println!("{}", e);
				Err(ZirconError::FailedToBind)
			}
		}
	}

	fn stop(&mut self) -> Result<(), ZirconError> {
		Ok(())
	}

	fn start_connection(&mut self, ident: String, address: String, port: i16, client_id: i64) {

	}

	fn stop_connection(&mut self, ident: String, reason: String) {

	}

	fn handle_packet(&mut self, address: String, port: i16, payload: Vec<i8>) {

	}

	fn player_manager(&self) -> &Box<PlayerManager> {
		&self.player_manager
	}
}
