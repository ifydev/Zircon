
use std::vec::Vec;

use server::player::Player;

pub enum ZirconError {
	FailedToBind
}

/// This is the base implementation of any server.
/// ZirconServer may be overridden in custom environments where certain requirements
/// are neededt that we don't provide.
pub trait ZirconServer {

	fn start(&mut self, address: &str, port: i16) -> Result<(), ZirconError>;
	fn stop(&mut self) -> Result<(), ZirconError>;

	fn online_players(&self) -> Vec<&dyn Player>;

	fn start_connection(&mut self, ident: String, address: String, port: i16, client_id: i64);
	fn stop_connection(&mut self, ident: String, reason: String);

	fn handle_packet(&mut self, address: String, port: i16, payload: Vec<i8>);
}

mod location;
mod server;

mod player;
