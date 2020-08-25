// define "one level"
pub const DIFFICULTY_MULTIPLIER: f32 = 100.;

pub struct DifficultyManager {
	difficulty: f32,
	last_difficulty: u32,
	min_difficulty: f32,
	max_difficulty: Option<f32>,
	difficulty_growth: f32,
	difficulty_drop: f32,
}

impl DifficultyManager {
	pub fn new(difficulty: f32, difficulty_growth: f32, difficulty_drop: f32) -> DifficultyManager {
		DifficultyManager {
			difficulty,
			last_difficulty: (difficulty * DIFFICULTY_MULTIPLIER) as u32,
			min_difficulty: difficulty,
			max_difficulty: None,
			difficulty_growth,
			difficulty_drop,
		}
	}

	pub fn get_difficulty(&self) -> f32 {
		self.difficulty
	}

	pub fn get_max_difficulty(&self) -> Option<f32> {
		self.max_difficulty
	}

	pub fn drop(&mut self) {
		self.difficulty -= self.difficulty_drop;
		if self.difficulty < 0. {
			self.difficulty = 0.;
		}
	}

	// return +- if difficulty * DIFFICULTY_MULTIPLIER just passed an integer
	// return true if high score is updated
	pub fn tick(&mut self, dt: f32) -> (std::cmp::Ordering, bool) {
		self.difficulty += self.difficulty_growth * dt;
		if self.difficulty < self.min_difficulty {
			self.min_difficulty = self.difficulty
		}
		let mut score_update = false;
		if self.difficulty > self.min_difficulty + 0.03
			&& self.difficulty > self.max_difficulty.unwrap_or(0.)
		{
			self.max_difficulty = Some(self.difficulty);
			score_update = true;
		}
		let level_change =
			((self.difficulty * DIFFICULTY_MULTIPLIER) as u32).cmp(&self.last_difficulty);
		self.last_difficulty = (self.difficulty * DIFFICULTY_MULTIPLIER) as u32;
		(level_change, score_update)
	}
}
