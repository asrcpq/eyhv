use crate::algebra::Point2f;
use crate::graphic_object::{generate_thick_arc, GraphicObjectsIntoIter};

// this is used for visualize, calculation only works as effects
pub struct StatusBar {
    // update data
    health_percent: f32,
    quick_percent: f32,
    slow_percent: f32,
    slowing: bool,
    shift: f32,
    player_p: Point2f,
    self_p: Point2f,

    rs: [f32; 3],
    rs_small: [f32; 3],
    rs_large: [f32; 3],

    split_angle: f32,
}

impl StatusBar {
    pub fn new() -> StatusBar {
        let rs_small = [70., 80., 90.];
        let rs_large = [180., 195., 210.];
        StatusBar {
            // these data should never be used
            health_percent: 0.,
            quick_percent: 0.,
            slow_percent: 0.,
            slowing: false,
            shift: 0.,
            player_p: Point2f::new(),
            self_p: Point2f::from_floats(250., 700.),
            rs: rs_small,
            rs_small,
            rs_large,
            split_angle: std::f32::consts::FRAC_PI_2,
        }
    }

    pub fn tick(
        &mut self,
        dt: f32,
        health_percent: f32,
        quick_percent: f32,
        slow_percent: f32,
        slowing: bool,
        player_p: Point2f,
    ) {
        self.health_percent = health_percent;
        self.quick_percent = quick_percent;
        self.slow_percent = slow_percent;
        self.player_p = player_p;
        self.self_p += (player_p - self.self_p) * dt * 20.;
        const MOMENTUM_SCALER: f32 = 0.5;
        self.self_p = self.self_p * MOMENTUM_SCALER + player_p * (1. - MOMENTUM_SCALER);
        for i in 0..3 {
            self.rs[i] = self.rs_small[i] * (1. - self.shift) + self.rs_large[i] * self.shift;
        }
        self.shift += (slowing as i32 as f32 - self.shift) * dt * 10.;

        let split_target = (1. - slowing as i32 as f32 * 2.) * std::f32::consts::FRAC_PI_2;
        const SPLIT_DIRECTION_THRESH: f32 = -std::f32::consts::FRAC_PI_2 + 0.5;
        if self.split_angle < SPLIT_DIRECTION_THRESH && !slowing {
            self.split_angle += 2. * std::f32::consts::PI;
        }
        self.split_angle += (split_target - self.split_angle) * dt * 10.;
        self.slowing = slowing;
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        const QUICK_SPLIT: f32 = 2. / 9. * std::f32::consts::PI;
        const SLOW_SPLIT: f32 = 16. / 9. * std::f32::consts::PI;
        let mut graphic_objects = generate_thick_arc(
            self.self_p,
            (self.rs[0], self.rs[1]),
            (0., &self.health_percent * 2. * std::f32::consts::PI),
            None,
            Some([
                if self.health_percent > 0.99 { 0.4 } else { 0.7 },
                0.4,
                0.4,
                0.3,
            ]),
        );
        graphic_objects.extend(generate_thick_arc(
            self.self_p,
            (self.rs[1], self.rs[2]),
            (
                self.split_angle,
                self.split_angle - self.quick_percent * QUICK_SPLIT,
            ),
            None,
            Some([
                0.4,
                1.0,
                0.3,
                if self.quick_percent > 0.99 { 0.2 } else { 0.4 },
            ]),
        ));
        graphic_objects.extend(generate_thick_arc(
            self.self_p,
            (self.rs[1], self.rs[2]),
            (
                self.split_angle,
                self.split_angle + self.slow_percent * SLOW_SPLIT,
            ),
            None,
            Some([0.5, 0.3, 1.0, 0.4 + 0.3 * self.shift]),
        ));
        graphic_objects.into_iter()
    }
}
