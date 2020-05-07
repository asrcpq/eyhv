use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter, LineSegs2f};

pub struct Background {
    lines_h: VecDeque<[f32; 6]>,
    lines_v: VecDeque<[f32; 6]>,

    timer: f32,
    line_count_h: u32,
    line_count_v: u32,
}

impl Background {
    pub fn new() -> Background {
        let mut lines_h = VecDeque::new();
        let mut lines_v = VecDeque::new();
        let line_count_h: u32 = 30;
        let line_count_v: u32 = 20;

        for x in 0..line_count_h + 1 {
            lines_h.push_back([x as f32, 0., 0., x as f32, line_count_v as f32, 0.]);
        }
        for x in 0..line_count_v + 1 {
            lines_v.push_back([0., x as f32, 0., line_count_h as f32, x as f32, 0.]);
        }

        Background {
            lines_h,
            lines_v,
            timer: 0.,
            line_count_h,
            line_count_v,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        const SPEED_SCALER: f32 = 2.;
        self.timer += dt;
        if self.timer >= 1. / SPEED_SCALER {
            self.timer -= 1. / SPEED_SCALER;
            self.lines_h.pop_front();
            self.lines_h.push_back([
                self.line_count_h as f32,
                0.,
                0.,
                self.line_count_h as f32,
                self.line_count_v as f32,
                0.,
            ]);
            self.lines_v.pop_front();
            self.lines_v.push_back([
                0.,
                self.line_count_v as f32,
                0.,
                self.line_count_h as f32,
                self.line_count_v as f32,
                0.,
            ]);
        }
        for line in self.lines_h.iter_mut() {
            line[0] -= dt * SPEED_SCALER;
            line[3] -= dt * SPEED_SCALER;
        }
        for line in self.lines_v.iter_mut() {
            line[1] -= dt * SPEED_SCALER;
            line[4] -= dt * SPEED_SCALER;
        }
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects: GraphicObjects = Default::default();
        const Y_FLIP: f32 = 800.;
        for line in self.lines_h.iter().chain(self.lines_v.iter()) {
            let mut vertices = Vec::new();
            let mut line_tmp = *line;
            line_tmp[0] *= 200.;
            line_tmp[1] *= 200.;
            line_tmp[3] *= 200.;
            line_tmp[4] *= 200.;
            line_tmp[2] += 1. + line_tmp[0] / 1000. + line_tmp[1] / 800.;
            line_tmp[5] += 1. + line_tmp[3] / 1000. + line_tmp[4] / 800.;
            //line_tmp[2] += 1.;
            //line_tmp[5] += 1.;
            vertices.push(Point2f::from_floats(
                line_tmp[0] / line_tmp[2],
                Y_FLIP - line_tmp[1] / line_tmp[2],
            ));
            vertices.push(Point2f::from_floats(
                line_tmp[3] / line_tmp[5],
                Y_FLIP - line_tmp[4] / line_tmp[5],
            ));
            graphic_objects.push(Box::new(LineSegs2f {
                vertices,
                color: [0., 0.7, 1.0, 0.3],
            }))
        }
        graphic_objects.into_iter()
    }
}
