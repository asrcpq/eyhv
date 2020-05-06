use crate::graphic_object::GraphicObjects;

pub struct SlowdownManager {
    quick: f32,
    slow: f32,
    quick_max: f32,
    slow_max: f32,
    quick_regen_rate: f32,
    slow_regen_rate: f32,
}

impl SlowdownManager {
    pub fn new() -> SlowdownMananger {
        const QUICK_MAX: f32 = 1.;
        const SLOW_MAX: f32 = 8.;
        SlowdownManager {
            quick: QUICK_MAX,
            slow: 0.,
            quick_max: QUICK_MAX,
            slow_max: SLOW_MAX,
            quick_regen_rate: 1.,
            slow_regen_rate: 0.3,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        // debug
        if self.quick > self.quick_max {
            panic!("quick value wrong!");
        }
        if self.slow > self.slow_max {
            panic!("slow value wrong!");
        }

        // cannot fill two pools in one frame to punish lag
        if self.quick < self.quick_max {
            self.quick = self.quick_max.min(self.quick + dt * self.quick_regen_rate);
        } else {
            self.slow = self.slow_max.min(self.slow + dt * self.slow_regen_rate);
        }
    }
}
