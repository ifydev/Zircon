
use server::player::Player;

use std::vec::Vec;

pub enum PlayerError {
	PlayerAlreadyLoggedIn,
	PlayerNotOnline
}

pub trait PlayerManager<'players> {

	fn online_players(&self) -> Vec<&dyn Player>;

	fn add_player(&mut self, player: &dyn Player) -> Result<(), PlayerError>;
	fn remove_player(&mut self, player: String) -> Result<(), PlayerError>;

	fn get_player_by_uuid(&self, uuid: String) -> Option<&dyn Player>;
	fn get_player_by_name(&self, name: String) -> Option<&dyn Player>;
}

pub struct DefaultPlayerManager {
	online_players: Vec<Box<Player>>
}

impl DefaultPlayerManager {

	pub fn new() -> Self {
		DefaultPlayerManager {
			online_players: Vec::new()
		}
	}
}

impl<'players> PlayerManager<'players> for DefaultPlayerManager {

	fn online_players(&self) -> Vec<&dyn Player> {
		// TODO: Figure out the cleanest way to return a reference of each player
		vec! []
	}

	fn add_player(&mut self, player: &dyn Player) -> Result<(), PlayerError> {
		Ok(())
	}

	fn remove_player(&mut self, player: String) -> Result<(), PlayerError> {
		Ok(())
	}

	fn get_player_by_uuid(&self, uuid: String) -> Option<&dyn Player> {
		None
	}

	fn get_player_by_name(&self, name: String) -> Option<&dyn Player> {
		None
	}
}
