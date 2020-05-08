use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter, LineSegs2f};

pub struct Background {
    lines_h: VecDeque<[f32; 6]>,
    lines_v: VecDeque<[f32; 6]>,

    timer_v: f32,
    timer_h: f32,
    line_count_h: u32,
    line_count_v: u32,

    shift: f32,
}

impl Background {
    pub fn new() -> Background {
        let mut lines_h = VecDeque::new();
        let mut lines_v = VecDeque::new();
        let line_count_h: u32 = 12;
        let line_count_v: u32 = 12;

        for x in 1..line_count_h + 1 {
            lines_h.push_back([x as f32, 0., 0., x as f32, line_count_v as f32, 0.]);
        }
        for x in 1..line_count_v + 1 {
            lines_v.push_back([0., x as f32, 0., line_count_h as f32, x as f32, 0.]);
        }

        Background {
            lines_h,
            lines_v,
            timer_v: 0.,
            timer_h: 0.,
            line_count_h,
            line_count_v,
            shift: 0.,
        }
    }

    pub fn tick(&mut self, dt: f32, slowing: bool) {
        const SPEED_SCALER_V: f32 = 4.;
        const SPEED_SCALER_H: f32 = 0.5;
        const SHIFT_SCALER: f32 = 10.;

        self.shift += (slowing as i32 as f32 - self.shift) * dt * SHIFT_SCALER;

        self.timer_v += dt;
        self.timer_h += dt;
        if self.timer_v >= 1. / SPEED_SCALER_V {
            self.timer_v -= 1. / SPEED_SCALER_V;
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
        if self.timer_h >= 1. / SPEED_SCALER_H {
            self.timer_h -= 1. / SPEED_SCALER_H;
            self.lines_h.pop_front();
            self.lines_h.push_back([
                self.line_count_h as f32,
                0.,
                0.,
                self.line_count_h as f32,
                self.line_count_v as f32,
                0.,
            ]);
        }
        for line in self.lines_h.iter_mut() {
            line[0] -= dt * SPEED_SCALER_H;
            line[3] -= dt * SPEED_SCALER_H;
        }
        for line in self.lines_v.iter_mut() {
            line[1] -= dt * SPEED_SCALER_V;
            line[4] -= dt * SPEED_SCALER_V;
        }
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        let mut graphic_objects: GraphicObjects = Default::default();
        const WINDOW_SCALER: f32 = 100.;
        const WINDOW_SHIFT_X: f32 = 200.;
        const WINDOW_SHIFT_Y: f32 = 350.;

        let c1 = 10f32.to_radians().cos();
        let s1 = 10f32.to_radians().sin();
        let c2 = 2f32.to_radians().cos();
        let s2 = 2f32.to_radians().sin();
        let c3 = 0f32.to_radians().cos();
        let s3 = 0f32.to_radians().sin();

        let each_z = [10. - self.shift * 2., 8. - self.shift * 2.];
        let each_x = [-49.5, -50.];
        let each_color = [
            [0.5, 0.0, 1., 0.4 + self.shift * 0.2],
            [0., 0.8, 1., 0.5 - self.shift * 0.2],
        ];

        for i in 0..2 {
            for line in self.lines_h.iter().chain(self.lines_v.iter()) {
                let mut vertices = Vec::new();
                let mut line_tmp = *line;
                line_tmp[0] *= 10.;
                line_tmp[1] *= 10.;
                line_tmp[3] *= 10.;
                line_tmp[4] *= 10.;
                line_tmp[2] += each_z[i];
                line_tmp[5] += each_z[i];
                line_tmp[0] += each_x[i];
                line_tmp[3] += each_x[i];
                line_tmp[1] -= 22.;
                line_tmp[4] -= 22.;

                macro_rules! x {
                    ($n: expr, $o: expr, $x: expr, $y: expr, $z: expr) => {
                        line_tmp[$n] =
                            line_tmp[$o] * $x + line_tmp[$o + 1] * $y + line_tmp[$o + 2] * $z;
                    };
                }

                for i in [0, 3].iter() {
                    x!(
                        *i,
                        *i,
                        c2 * c3,
                        s1 * s2 * c3 - c1 * s3,
                        c1 * s2 * c3 + s1 * s3
                    );
                    x!(
                        i + 1,
                        *i,
                        c2 * s3,
                        s1 * s2 * s3 + c1 * c3,
                        c1 * s2 * s3 - s1 * c3
                    );
                    x!(i + 2, *i, -s2, s1 * c2, c1 * c2);
                }
                if line_tmp[2] < 1. || line_tmp[5] < 1. {
                    continue;
                }

                vertices.push(Point2f::from_floats(
                    WINDOW_SCALER * (line_tmp[0] / line_tmp[2]) + WINDOW_SHIFT_X,
                    WINDOW_SCALER * (-line_tmp[1] / line_tmp[2]) + WINDOW_SHIFT_Y,
                ));
                vertices.push(Point2f::from_floats(
                    WINDOW_SCALER * (line_tmp[3] / line_tmp[5]) + WINDOW_SHIFT_X,
                    WINDOW_SCALER * (-line_tmp[4] / line_tmp[5]) + WINDOW_SHIFT_Y,
                ));
                graphic_objects.push(Box::new(LineSegs2f {
                    vertices,
                    color: each_color[i],
                }))
            }
        }
        graphic_objects.into_iter()
    }
}
