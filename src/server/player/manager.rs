
use server::player::Player;

use std::vec::Vec;

pub enum PlayerError {
	PlayerAlreadyLoggedIn,
	PlayerNotOnline
}

pub trait PlayerManager {

	fn online_players(&self) -> Vec<&dyn Player>;

	fn add_player(&mut self, player: &dyn Player) -> Result<(), PlayerError>;
	fn remove_player(&mut self, player: String) -> Result<(), PlayerError>;

	fn get_player_by_uuid(&self, uuid: String) -> Option<&dyn Player>;
	fn get_player_by_name(&self, name: String) -> Option<&dyn Player>;

	fn max_players(&self) -> usize;
	fn players_on(&self) -> usize;
}

pub struct DefaultPlayerManager {
	players: Vec<Box<Player>>,
	max_players: usize
}

impl DefaultPlayerManager {

	pub fn new(max_players: usize) -> Self {
		DefaultPlayerManager {
			players: Vec::new(),
			max_players
		}
	}
}

impl PlayerManager for DefaultPlayerManager {

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
		for player in self.online_players() {
			if player.uuid() == uuid {
				return Some(player);
			}
		}
		return None
	}

	fn get_player_by_name(&self, name: String) -> Option<&dyn Player> {
		for player in self.online_players() {
			if player.name() == name {
				return Some(player);
			}
		}
		return None
	}

	fn max_players(&self) -> usize {
		self.max_players
	}

	fn players_on(&self) -> usize {
		self.online_players().len()
	}
}
