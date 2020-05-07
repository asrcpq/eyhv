use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter};

pub struct Background {
    warp_rails_l: (Point2f, Point2f),
    warp_rails_r: (Point2f, Point2f),
    timer: f32,
    cycle: f32,
}

impl Background {
    pub fn new() -> Background {
        Background {
            warp_rails_l: (
                Point2f::from_floats(-0.5, 0.5),
                Point2f::from_floats(0., 0.),
            ),
            warp_rails_r: (Point2f::from_floats(0.5, 1.5), Point2f::from_floats(1., 1.)),
            timer: 0.,
            cycle: 1.,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        self.timer += dt;
        while self.timer > self.cycle {
            self.timer -= self.cycle;
        }
    }

    #[inline]
    fn t_to_k(t: f32) -> f32 {
        (-t).exp()
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        //const MAX_WARPS: u32 = 20;
        //for i in 0..MAX_WARPS {
        //    Self::t_to_k()
        //}
        let mut graphic_objects: GraphicObjects = Default::default();
        graphic_objects.into_iter()
    }
}
