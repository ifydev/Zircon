
use server::location::Location;
use vector::Vector2;

pub trait Player {

	fn name() -> String;
	fn uuid() -> String; // TODO: Are we using UUIDs here? Need to check proto
	
	// TODO: Verify that f32 is right here
	fn location() -> Location;
	fn rotation() -> Vector2<f32>;
}
