use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObjects, GraphicObjectsIntoIter, LineSegs2f, Polygon2f};

pub struct Background {
    lines_h: VecDeque<Vec<Vec<[f32; 3]>>>,
    lines_v: VecDeque<Vec<Vec<[f32; 3]>>>,
    lines_font: VecDeque<Vec<Vec<[f32; 3]>>>,

    timer_v: f32,
    timer_h: f32, line_count_h: u32, line_count_v: u32,

    shift: f32,

    message_queue: VecDeque<String>,
    message_queue_max: usize,
}

impl Background {
    pub fn new() -> Background {
        let mut lines_h = VecDeque::new();
        let mut lines_v = VecDeque::new();
        let mut lines_font = VecDeque::new();
        let line_count_h: u32 = 12;
        let line_count_v: u32 = 12;

        for x in 1..line_count_h + 1 {
            lines_h.push_back(vec![vec![
                [x as f32, 0., 0.],
                [x as f32, line_count_v as f32, 0.],
            ]]);
        }
        for x in 1..line_count_v + 1 {
            lines_v.push_back(vec![vec![
                [0., x as f32, 0.],
                [line_count_h as f32, x as f32, 0.],
            ]]);
        }
        for _ in 0..12 * 12 {
            lines_font.push_back(Vec::new());
        }

        Background {
            lines_h,
            lines_v,
            lines_font,
            timer_v: 0.,
            timer_h: 0.,
            line_count_h,
            line_count_v,
            shift: 0.,
            message_queue: VecDeque::new(),
            message_queue_max: 32,
        }
    }

    pub fn send_message(&mut self, string: String) {
        if self.message_queue.len() >= self.message_queue_max {
            println!("Message: \"{}\" overflow!", string);
            return;
        }
        assert_eq!(string.len(), 12);
        self.message_queue.push_back(string);
    }

    pub fn tick(&mut self, dt: f32, slowing: bool) {
        const SPEED_SCALER_V: f32 = 4.;
        const SPEED_SCALER_H: f32 = 0.5;
        const SHIFT_SCALER: f32 = 10.;

        self.shift += (slowing as i32 as f32 - self.shift) * dt * SHIFT_SCALER;

        self.timer_v += dt;
        self.timer_h += dt;
        for line in self.lines_h.iter_mut() {
            for vertex in line[0].iter_mut() {
                vertex[0] -= dt * SPEED_SCALER_H;
            }
        }
        for line in self.lines_v.iter_mut() {
            for vertex in line[0].iter_mut() {
                vertex[1] -= dt * SPEED_SCALER_V;
            }
        }
        for lines in self.lines_font.iter_mut() {
            for line in lines.iter_mut() {
                for vertex in line.iter_mut() {
                    vertex[0] -= dt * SPEED_SCALER_H;
                    vertex[1] -= dt * SPEED_SCALER_V;
                }
            }
        }
        if self.timer_v >= 1. / SPEED_SCALER_V {
            if let Some(string) = self.message_queue.pop_front() {
                let mut id: f32 = -self.timer_h * SPEED_SCALER_H;
                for ch in string.chars() {
                    let mut lines = Vec::new();
                    let graphic_objects = mray::fsd::fsd(ch);
                    for graphic_object in graphic_objects.clone().into_iter() {
                        let mut vertices = Vec::new();
                        if let Some(line_segs) = graphic_object.as_any().downcast_ref::<Polygon2f>()
                        {
                            for vertex in line_segs.vertices.iter() {
                                vertices.push([
                                    id + vertex.x,
                                    self.line_count_v as f32 + 1. - vertex.y,
                                    0.,
                                ]);
                            }
                        } else {
                            unreachable!();
                        }
                        lines.push(vertices);
                    }
                    self.lines_font.push_back(lines);
                    //println!("{:?}", self.lines_font);
                    id += 1.;
                }
            } else {
                for _ in 0..12 {
                    self.lines_font.push_back(Vec::new());
                }
            }
            for _ in 0..12 {
                self.lines_font.pop_front();
            }

            self.timer_v -= 1. / SPEED_SCALER_V;
            self.lines_v.pop_front();
            self.lines_v.push_back(vec![vec![
                [0., self.line_count_v as f32, 0.],
                [self.line_count_h as f32, self.line_count_v as f32, 0.],
            ]]);
        }
        if self.timer_h >= 1. / SPEED_SCALER_H {
            self.timer_h -= 1. / SPEED_SCALER_H;
            self.lines_h.pop_front();
            self.lines_h.push_back(vec![vec![
                [self.line_count_h as f32, 0., 0.],
                [self.line_count_h as f32, self.line_count_v as f32, 0.],
            ]]);
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

        let line_shift = [
            [-49.5, -22., 10. - self.shift * 3.],
            [-49.5, -22., 10. - self.shift * 3.],
            [-50., -22., 8. - self.shift * 1.2],
            [-50., -22., 8. - self.shift * 1.2],
            [-50., -22., 8. - self.shift * 1.2],
        ];

        let each_color = [
            [
                0.7,
                0.8 - self.shift * 0.8,
                0.2 + self.shift * 0.8,
                0.45 + self.shift * 0.2,
            ],
            [
                0.7,
                0.8 - self.shift * 0.8,
                0.2 + self.shift * 0.8,
                0.45 + self.shift * 0.2,
            ],
            [
                1. - (self.shift - 0.5).abs() * 2.,
                1. - (self.shift - 0.5).abs() * 2.,
                1.,
                0.55,
            ],
            [
                1. - (self.shift - 0.5).abs() * 2.,
                1. - (self.shift - 0.5).abs() * 2.,
                1.,
                0.55,
            ],
            [1., 1., 1., 0.3],
        ];

        for layer in 0..5 {
            'line_segs: for lines in match layer {
                0 => self.lines_h.iter(),
                1 => self.lines_v.iter(),
                2 => self.lines_h.iter(),
                3 => self.lines_v.iter(),
                4 => self.lines_font.iter(),
                _ => unreachable!(),
            } {
                let mut lines_tmp = lines.clone();
                for line in lines_tmp.iter_mut() {
                    for vertex in line.iter_mut() {
                        vertex[0] *= 10.;
                        vertex[1] *= 10.;
                        for (i, axis) in vertex.iter_mut().enumerate() {
                            *axis += line_shift[layer][i];
                        }
                    }
                }

                for line in lines_tmp.iter_mut() {
                    for vertex in line.iter_mut() {
                        macro_rules! x {
                            ($n: expr, $x: expr, $y: expr, $z: expr) => {
                                vertex[$n] = vertex[0] * $x + vertex[1] * $y + vertex[2] * $z;
                            };
                        }
                        x!(0, c2 * c3, s1 * s2 * c3 - c1 * s3, c1 * s2 * c3 + s1 * s3);
                        x!(1, c2 * s3, s1 * s2 * s3 + c1 * c3, c1 * s2 * s3 - s1 * c3);
                        x!(2, -s2, s1 * c2, c1 * c2);
                        if vertex[2] < 0.1 {
                            continue 'line_segs;
                        }
                    }
                }

                for line in lines_tmp.iter_mut() {
                    let mut vertices = Vec::new();
                    for vertex in line.iter_mut() {
                        vertices.push(Point2f::from_floats(
                            WINDOW_SCALER * (vertex[0] / vertex[2]) + WINDOW_SHIFT_X,
                            WINDOW_SCALER * (-vertex[1] / vertex[2]) + WINDOW_SHIFT_Y,
                        ));
                    }
                    if layer == 4 {
                        graphic_objects.push(Box::new(Polygon2f {
                            vertices,
                            color: each_color[layer],
                            border_color: each_color[layer],
                        }))
                    } else {
                        graphic_objects.push(Box::new(LineSegs2f {
                            vertices,
                            color: each_color[layer],
                        }))
                    }
                }
            }
        }
        graphic_objects.into_iter()
    }
}
