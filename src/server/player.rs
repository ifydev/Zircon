
use server::location::Location;
use vector::Vector2;

pub trait Player {

	fn name(&self) -> String;
	fn uuid(&self) -> String; // TODO: Are we using UUIDs here? Need to check proto
	
	// TODO: Verify that f32 is right here
	fn location(&self) -> Location;
	fn rotation(&self) -> Vector2<f32>;
}

pub struct DefaultPlayer {
	name: String,
	uuid: String,

	location: Location,
	rotation: Vector2<f32>
}

impl Player for DefaultPlayer {

	fn name(&self) -> String {
		self.name.clone()
	}

	fn uuid(&self) -> String {
		self.uuid.clone()
	}

	fn location(&self) -> Location {
		self.location.clone()
	}

	fn rotation(&self) -> Vector2<f32> {
		self.rotation.clone()
	}
}
