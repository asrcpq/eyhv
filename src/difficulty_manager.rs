// define "one level"
pub const DIFFICULTY_MULTIPLIER: f32 = 100.;

pub struct DifficultyManager {
	difficulty: f32,
	last_difficulty: u32,
	max_difficulty: f32,
	difficulty_growth: f32,
	difficulty_drop: f32,
}

impl DifficultyManager {
	pub fn new(difficulty: f32, difficulty_growth: f32, difficulty_drop: f32) -> DifficultyManager {
		DifficultyManager {
			difficulty,
			last_difficulty: (difficulty * DIFFICULTY_MULTIPLIER) as u32,
			max_difficulty: difficulty,
			difficulty_growth,
			difficulty_drop,
		}
	}

	pub fn get_difficulty(&self) -> f32 {
		self.difficulty
	}

	pub fn get_max_difficulty(&self) -> f32 {
		self.max_difficulty
	}

	pub fn drop(&mut self) {
		self.difficulty -= self.difficulty_drop;
		if self.difficulty < 0. {
			self.difficulty = 0.;
		}
	}

	// return Some(+-) if difficulty * DIFFICULTY_MULTIPLIER just passed an integer
	pub fn tick(&mut self, dt: f32) -> std::cmp::Ordering {
		self.difficulty += self.difficulty_growth * dt;
		if self.difficulty > self.max_difficulty {
			self.max_difficulty = self.difficulty;
		}
		let result = ((self.difficulty * DIFFICULTY_MULTIPLIER) as u32).cmp(&self.last_difficulty);
		self.last_difficulty = (self.difficulty * DIFFICULTY_MULTIPLIER) as u32;
		result
	}
}
