use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjectsIntoIter, generate_thick_arc};

// this is used for visualize, calculation only works as effects
pub struct StatusBar {
    // update data
    health_percent: f32,
    quick_percent: f32,
    slow_percent: f32,
    slowing: bool,
    player_p: Point2f,
}

impl StatusBar {
    pub fn new() -> StatusBar {
        StatusBar {
            // these data should never be used
            health_percent: 0.,
            quick_percent: 0.,
            slow_percent: 0.,
            slowing: false,
            player_p: Point2f::new(),
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
        self.slowing = slowing;
        self.player_p = player_p;
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects = generate_thick_arc(
            self.player_p,
            (70., 80.),
            (0., &self.health_percent * 2. * std::f32::consts::PI),
            None,
            Some([0.2, 0.5, 1.0, 0.3]),
        );
        graphic_objects.extend(generate_thick_arc(
            self.player_p,
            (80., 90.),
            (0., &self.quick_percent * 2. * std::f32::consts::PI),
            None,
            Some([0.4, 1.0, 0.3, 0.3]),
        ));
        graphic_objects.extend(generate_thick_arc(
            self.player_p,
            (90., 100.),
            (0., &self.slow_percent * 2. * std::f32::consts::PI),
            None,
            Some([0.5, 0.3, 1.0, 0.4]),
        ));
        graphic_objects.into_iter()
    }
}
