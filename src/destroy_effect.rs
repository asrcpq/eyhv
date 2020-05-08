use std::collections::VecDeque;

use crate::algebra::Point2f;
use crate::graphic_object::{GraphicObject, GraphicObjects, GraphicObjectsIntoIter, LineSegs2f};

use rand::Rng;
use rand::SeedableRng;

pub struct DestroyedObjects {
    // speed, line
    lines: VecDeque<(Point2f, LineSegs2f)>,
    rng: rand_pcg::Pcg64Mcg,
}

impl DestroyedObjects {
    pub fn new(seed: u64) -> DestroyedObjects {
        DestroyedObjects {
            lines: VecDeque::new(),
            rng: rand_pcg::Pcg64Mcg::seed_from_u64(seed),
        }
    }

    // for memleak monitor
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn tick(&mut self, dt: f32) {
        const D_ALPHA: f32 = 0.5;
        let len = self.lines.len();
        for _ in 0..len {
            let mut line = self.lines.pop_front().unwrap();
            line.1.color[3] -= dt * D_ALPHA;
            if line.1.color[3] <= 0. {
                continue;
            }
            line.1 = line.1.shift(line.0);
            self.lines.push_back(line);
        }
    }

    pub fn push(&mut self, graphic_objects: GraphicObjects) {
        let move_direction = Point2f::from_floats(0., -2.);
        for graphic_object in graphic_objects.into_iter() {
            if let Some(line_segs) = graphic_object.as_any().downcast_ref::<LineSegs2f>() {
                let mut iter = line_segs.vertices.iter();
                let mut last_vertex = match iter.next() {
                    None => continue,
                    Some(vertex) => vertex,
                };
                for vertex in iter {
                    self.lines.push_back((
                        Point2f::from_floats(
                            (self.rng.gen_range(-1., 1.) as f32).powi(3) * 6.,
                            (self.rng.gen_range(-1., 1.) as f32).powi(3) * 6.,
                        ) + move_direction,
                        LineSegs2f {
                            vertices: vec![*last_vertex, *vertex],
                            color: line_segs.color,
                        },
                    ));
                    last_vertex = vertex;
                }
            }
        }
    }

    pub fn graphic_objects_iter(&self) -> GraphicObjectsIntoIter {
        GraphicObjects::new(
            self.lines
                .iter()
                .map(|x| Box::<dyn GraphicObject>::from(Box::new(x.1.clone())))
                .collect(),
        )
        .into_iter()
    }
}
