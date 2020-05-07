pub struct SlowdownManager {
    quick: f32,
    slow: f32,
    slowing: bool,
    quick_max: f32,
    slow_max: f32,
    // cannot use slow pool if under threshold value
    quick_replenish_rate: f32,
    slow_replenish_rate: f32,
}

impl SlowdownManager {
    pub fn new() -> SlowdownManager {
        const QUICK_MAX: f32 = 1.;
        const SLOW_MAX: f32 = 8.;
        SlowdownManager {
            quick: QUICK_MAX,
            slow: 0.,
            slowing: false,
            quick_max: QUICK_MAX,
            slow_max: SLOW_MAX,
            quick_replenish_rate: 1.,
            slow_replenish_rate: 0.3,
        }
    }

    // return slowing status
    pub fn switch(&mut self, switch: bool) -> bool {
        const EPS: f32 = 0.1;
        if self.slowing && !switch {
            self.slowing = false;
            false
        } else {
            // prevent setting frequently
            if self.quick > EPS || self.slow > EPS {
                self.slowing = true;
                true
            } else {
                false
            }
        }
    }

    pub fn tick(&mut self, dt: f32) -> bool {
        // println!("{}\t{}", self.quick, self.slow);
        // debug
        if self.quick > self.quick_max {
            panic!("quick value wrong!");
        }
        if self.slow > self.slow_max {
            panic!("slow value wrong!");
        }

        // not filling/consuming two pools in one frame for simplicity
        if self.slowing {
            if self.quick > 0. {
                self.quick = 0f32.max(self.quick - dt);
            } else if self.slow > 0. {
                self.slow = 0f32.max(self.slow - dt);
            } else {
                self.slowing = false;
            }
            return self.slowing;
        }

        if self.quick < self.quick_max {
            self.quick = self
                .quick_max
                .min(self.quick + dt * self.quick_replenish_rate);
        } else {
            self.slow = self.slow_max.min(self.slow + dt * self.slow_replenish_rate);
        }
        false
    }

    pub fn get_info(&self) -> (f32, f32, bool) {
        (
            self.quick / self.quick_max,
            self.slow / self.slow_max,
            self.slowing,
        )
    }
}
