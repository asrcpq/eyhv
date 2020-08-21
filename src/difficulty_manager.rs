// define "one level"
pub const DIFFICULTY_MULTIPLIER: f32 = 100.;

pub struct DifficultyManager {
    difficulty: f32,
    difficulty_growth: f32,
    difficulty_drop: f32,
}

impl DifficultyManager {
    pub fn new(difficulty: f32, difficulty_growth: f32, difficulty_drop: f32) -> DifficultyManager {
        DifficultyManager {
            difficulty,
            difficulty_growth,
            difficulty_drop,
        }
    }

    pub fn get_difficulty(&self) -> f32 {
        self.difficulty
    }

    pub fn drop(&mut self) {
        self.difficulty -= self.difficulty_drop;
        if self.difficulty < 0. {
            self.difficulty = 0.;
        }
    }

    // return true if difficulty * DIFFICULTY_MULTIPLIER just passed an integer
    pub fn tick(&mut self, dt: f32) -> bool {
        let last_difficulty: u32 = (self.difficulty * DIFFICULTY_MULTIPLIER) as u32;
        self.difficulty += self.difficulty_growth * dt;
        (self.difficulty * DIFFICULTY_MULTIPLIER) as u32 != last_difficulty
    }
}
