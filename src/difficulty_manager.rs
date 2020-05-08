pub struct DifficultyManager {
    difficulty: f32,
    difficulty_growth: f32,
}

impl DifficultyManager {
    pub fn new(difficulty: f32, difficulty_growth: f32) -> DifficultyManager {
        DifficultyManager {
            difficulty,
            difficulty_growth,
        }
    }

    pub fn get_difficulty(&self) -> f32 {
        self.difficulty
    }

    // return true if difficulty * 100 just passed an integer
    pub fn tick(&mut self, dt: f32, player_health_percent: f32) -> bool{
        let last_difficulty: u32 = (self.difficulty * 100.) as u32;
        self.difficulty += self.difficulty_growth * dt * player_health_percent;
        (self.difficulty * 100.) as u32 > last_difficulty
    }
}
