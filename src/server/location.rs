
pub struct Location {
	x: f32,
	y: f32,
	z: f32,

	world: String  // TODO: Replace with a real `World` type
}

impl Location {

	pub fn new(x: f32, y: f32, z: f32, world: &str) -> Self {
		Location {
			x,
			y,
			z,

			world: world.to_string()
		}
	}

	pub fn x(&self) -> f32 {
		self.x
	}

	pub fn y(&self) -> f32 {
		self.y
	}

	pub fn z(&self) -> f32 {
		self.z
	}

	pub fn block_x(&self) -> i32 {
		self.x.floor() as i32
	}

	pub fn block_y(&self) -> i32 {
		self.y.floor() as i32
	}

	pub fn block_z(&self) -> i32 {
		self.z.floor() as i32
	}

	pub fn world(&self) -> String {
		self.world.clone()
	}
}
