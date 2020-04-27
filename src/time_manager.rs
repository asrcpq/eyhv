pub struct TimeManager {
    dt_scaler: f32,
    slowdown: bool,
    shifting: bool,
    // absolute value of scaler per second, not percentage of difference
    shift_rate_down: f32,
    shift_rate_up: f32,
    dt_scaler_normal: f32,
    dt_scaler_slow: f32,
}

impl TimeManager {
    pub fn new() -> TimeManager {
        TimeManager {
            dt_scaler: 1.,
            slowdown: false,
            shifting: false,
            shift_rate_down: 2.5,
            shift_rate_up: 5.,
            dt_scaler_normal: 1.,
            dt_scaler_slow: 0.5,
        }
    }

    pub fn set_state(&mut self, slowdown: bool) {
        self.slowdown = slowdown;
        if slowdown && self.dt_scaler > self.dt_scaler_slow
            || !slowdown && self.dt_scaler < self.dt_scaler_normal
        {
            self.shifting = true
        }
    }

    fn update_scaler(&mut self, dt: f32) {
        if self.shifting {
            if self.slowdown {
                self.dt_scaler -= self.shift_rate_down * dt;
                if self.dt_scaler < self.dt_scaler_slow {
                    self.dt_scaler = self.dt_scaler_slow;
                    self.shifting = false
                }
            } else {
                self.dt_scaler += self.shift_rate_up * dt;
                if self.dt_scaler > self.dt_scaler_normal {
                    self.dt_scaler = self.dt_scaler_normal;
                    self.shifting = false
                }
            }
        }
    }

    pub fn update_and_get_dt_scaler(&mut self, dt: f32) -> f32 {
        self.update_scaler(dt);
        self.dt_scaler
    }
}
