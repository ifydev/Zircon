
use server::{
	player::Player,
	ZirconServer, ZirconError
};
use std::net::UdpSocket;

pub struct DefaultServer<'players> {
	socket: UdpSocket,
	players: Vec<&'players dyn Player>
}

impl<'players> ZirconServer for DefaultServer<'players> {

	fn start(&mut self, address: &str, port: u16) -> Result<(), ZirconError> {
		match UdpSocket::bind(&format!("{}:{}", address, port)) {
			Ok(socket) => {
				self.socket = socket;
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
}
