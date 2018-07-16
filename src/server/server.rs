
use server::{
	player::Player,
	ZirconServer, ZirconError
};

use std::{
	net::UdpSocket,
	vec::Vec
};

pub struct DefaultServer<'players> {
	socket: Option<UdpSocket>,
	players: Vec<&'players dyn Player>,
	max_ticks: u8,
	max_players: u32
}

impl DefaultServer {

	pub fn new(max_players: u32) -> Self {
		socket: None,
		players: vec! [],
		max_ticks: 20,
		max_players
	}
}

impl<'players> ZirconServer for DefaultServer<'players> {

	fn start(&mut self, address: &str, port: u16) -> Result<(), ZirconError> {
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

	fn online_players(&self) -> Vec<&dyn Player> {
		self.players.clone()
	}

	fn start_connection(&mut self, ident: String, address: String, port: i16, client_id: i64) {

	}

	fn stop_connection(&mut self, ident: String, reason: String) {

	}

	fn handle_packet(&mut self, address: String, port: i16, payload: Vec<i8>)
}
