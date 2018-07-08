
use server::location::Location;
use vector::Vector2;

pub trait Player {

	fn name(&self) -> String;
	fn uuid(&self) -> String; // TODO: Are we using UUIDs here? Need to check proto
	
	// TODO: Verify that f32 is right here
	fn location(&self) -> Location;
	fn rotation(&self) -> Vector2<f32>;
}
