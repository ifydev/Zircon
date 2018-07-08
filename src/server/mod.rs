
use std::vec::Vec;

use server::player::Player;

pub enum ZirconError {
	FailedToBind
}

/// This is the base implementation of any server.
/// ZirconServer may be overridden in custom environments where certain requirements
/// are neededt that we don't provide.
pub trait ZirconServer {

	fn start(&self, address: &str, port: u16) -> Result<(), ZirconError>;
	fn stop(&self) -> Result<(), ZirconError>;

	fn online_players(&self) -> Vec<&dyn Player>;
}

mod player;
mod location;
