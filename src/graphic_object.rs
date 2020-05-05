use crate::algebra::{Mat2x2f, Point2f};
use crate::canvas::Canvas;
use dyn_clone::DynClone;
use std::any::Any;

use crate::window_rect::WINDOW_RECT;

#[derive(Clone, Debug)]
pub struct LineSegs2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4], // rgba
}
impl LineSegs2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4]) -> LineSegs2f {
        LineSegs2f { vertices, color }
    }

    pub fn from_floats(floats: Vec<f32>) -> LineSegs2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        let r = iter.next().unwrap();
        let g = iter.next().unwrap();
        let b = iter.next().unwrap();
        let a = iter.next().unwrap();
        let color: [f32; 4] = [*r, *g, *b, *a];
        while match iter.next() {
            Some(v1) => match iter.next() {
                Some(v2) => {
                    vertices.push(Point2f::from_floats(*v1, *v2));
                    true
                }
                None => panic!("odd parse"),
            },
            None => false,
        } {}
        LineSegs2f::new(vertices, color)
    }

    #[inline]
    fn wu(x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 4], canvas: &mut Canvas) {
        let mut x1: i32 = x1.round() as i32;
        let mut y1: i32 = y1.round() as i32;
        let mut x2: i32 = x2.round() as i32;
        let mut y2: i32 = y2.round() as i32;
        let mut dx = x2 - x1;
        let dy = y2 - y1;
        if dx == 0 {
            if dy < 0 {
                let t = y1;
                y1 = y2;
                y2 = t;
            }
            for y in y1..y2 + 1 {
                canvas.putpixel(x1, y, color);
            }
            return;
        }

        if dy == 0 {
            if dx < 0 {
                let t = x1;
                x1 = x2;
                x2 = t;
            }
            for x in x1..x2 + 1 {
                canvas.putpixel(x, y1, color);
            }
            return;
        }

        if dx == dy {
            if dx < 0 {
                let t = x1;
                x1 = x2;
                x2 = t;
                let t = y1;
                y1 = y2;
                y2 = t;
                dx = -dx;
            }
            for i in 0..dx + 1 {
                canvas.putpixel(x1 + i, y1 + i, color);
            }
            return;
        }

        if dx == -dy {
            if dx < 0 {
                let t = x1;
                x1 = x2;
                x2 = t;
                let t = y1;
                y1 = y2;
                y2 = t;
                dx = -dx;
            }
            for i in 0..dx + 1 {
                canvas.putpixel(x1 + i, y1 - i, color);
            }
            return;
        }

        let k = dy as f32 / dx as f32;
        let mut e: f32 = 0.;

        if dx + dy < 0 {
            let t = x1;
            x1 = x2;
            x2 = t;
            let t = y1;
            y1 = y2;
            y2 = t;
        }

        if k > 0. && k < 1. {
            let mut py = y1;
            for px in x1..x2 {
                canvas.putpixel(px, py, [color[0], color[1], color[2], color[3] * (1. - e)]);
                canvas.putpixel(px, py + 1, [color[0], color[1], color[2], color[3] * e]);
                e += k;
                if e >= 1. {
                    py += 1;
                    e -= 1.;
                }
            }
        } else if k > 1. {
            let mut px = x1;
            for py in y1..y2 {
                canvas.putpixel(px, py, [color[0], color[1], color[2], color[3] * (1. - e)]);
                canvas.putpixel(px + 1, py, [color[0], color[1], color[2], color[3] * e]);
                e += 1. / k;
                if e >= 1. {
                    px += 1;
                    e -= 1.;
                }
            }
        }

        else if k > -1. && k < 0. {
            let mut py = y1;
            for px in x1..x2 {
                canvas.putpixel(px, py, [color[0], color[1], color[2], color[3] * (1. + e)]);
                canvas.putpixel(px, py - 1, [color[0], color[1], color[2], color[3] * -e]);
                e += k;
                if e <= -1. {
                    py -= 1;
                    e += 1.0;
                }
            }
        } else if k < -1. {
            let mut px = x2;
            for py in (y1..y2).rev() {
                canvas.putpixel(px, py, [color[0], color[1], color[2], color[3] * (1. - e)]);
                canvas.putpixel(px + 1, py, [color[0], color[1], color[2], color[3] * e]);
                e += -1. / k;
                if e >= 1. {
                    px += 1;
                    e -= 1.;
                }
            }
        }
    }

    #[inline]
    fn bresenham(x1: f32, y1: f32, x2: f32, y2: f32, color: [f32; 4], canvas: &mut Canvas) {
        let mut x: i32;
        let mut y: i32;
        let mut dx: i32;
        let mut dy: i32;
        let mut s1: i32;
        let mut s2: i32;
        let mut p: i32;
        let mut temp: i32;
        let mut interchange: i32;
        x = x1 as i32;
        y = y1 as i32;
        dx=(x2-x1).abs() as i32;
        dy=(y2-y1).abs() as i32;

        if x2 > x1 {
            s1 = 1;
        } else {
            s1 = -1;
        }

        if y2 > y1 {
            s2 = 1;
        } else {
            s2 = -1;
        }

        if dy > dx {
            temp = dx;
            dx = dy;
            dy = temp;
            interchange = 1;
        } else {
            interchange = 0;
        }

        p = 2 * dy - dx;
        for _ in 1..dx + 1 {
            canvas.putpixel(x, y, color);
            if p >= 0 {
                if interchange == 0 {
                    y = y + s2;
                } else {
                    x = x + s1;
                }
                p = p - 2 * dx;
            }
            if interchange == 0 {
                x = x + s1; 
            } else {
                y = y + s2;
            }
            p = p + 2 * dy;
        }
    }
}

pub trait GraphicObject: DynClone + Sync + Any {
    fn as_any(&self) -> &dyn Any;
    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject>;
    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject>;
    fn zoom(&self, k: f32) -> Box<dyn GraphicObject>;

    fn render(&self, canvas: &mut Canvas);
}

dyn_clone::clone_trait_object!(GraphicObject);

impl GraphicObject for LineSegs2f {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| *x + dp).collect(),
            color: self.color,
        })
    }

    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| rotate_mat * *x).collect(),
            color: self.color,
        })
    }

    fn zoom(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(LineSegs2f {
            vertices: self.vertices.iter().map(|x| *x * k).collect(),
            color: self.color,
        })
    }

    fn render(&self, mut canvas: &mut Canvas) {
        let mut flag = false;
        let mut x1: f32 = 0.; // convince compiler
        let mut x2: f32;
        let mut y1: f32 = 0.; // convince compiler
        let mut y2: f32;
        for vertice in self.vertices.iter() {
            if !WINDOW_RECT.contain(*vertice) {
                continue;
            }

            if !flag {
                flag = true;
                x1 = vertice.x;
                y1 = vertice.y;
            } else {
                x2  = vertice.x;
                y2  = vertice.y;
                LineSegs2f::wu(x1, y1, x2, y2, self.color, &mut canvas);
                x1 = x2;
                y1 = y2;
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Polygon2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4],
}

impl GraphicObject for Polygon2f {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shift(&self, dp: Point2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x + dp).collect(),
            color: self.color,
        })
    }

    fn rotate(&self, rotate_mat: Mat2x2f) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| rotate_mat * *x).collect(),
            color: self.color,
        })
    }

    fn zoom(&self, k: f32) -> Box<dyn GraphicObject> {
        Box::new(Polygon2f {
            vertices: self.vertices.iter().map(|x| *x * k).collect(),
            color: self.color,
        })
    }

    fn render(&self, canvas: &mut Canvas) {
    }
}

impl Polygon2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4]) -> Polygon2f {
        Polygon2f { vertices, color }
    }

    pub fn from_floats(floats: Vec<f32>) -> Polygon2f {
        let mut vertices: Vec<Point2f> = Vec::new();
        let mut iter = floats.iter();
        let r = iter.next().unwrap();
        let g = iter.next().unwrap();
        let b = iter.next().unwrap();
        let a = iter.next().unwrap();
        let color: [f32; 4] = [*r, *g, *b, *a];
        while match iter.next() {
            Some(v1) => match iter.next() {
                Some(v2) => {
                    vertices.push(Point2f::from_floats(*v1, *v2));
                    true
                }
                None => panic!("odd parse"),
            },
            None => false,
        } {}
        Polygon2f::new(vertices, color)
    }
}

#[derive(Clone, Default)]
pub struct GraphicObjects {
    graphic_objects: Vec<Box<dyn GraphicObject>>,
}

impl GraphicObjects {
    pub fn shift(&self, point2f: Point2f) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.shift(point2f))
                .collect(),
        }
    }

    pub fn rotate(&self, rotate_mat: Mat2x2f) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.rotate(rotate_mat))
                .collect(),
        }
    }

    pub fn zoom(&self, k: f32) -> GraphicObjects {
        GraphicObjects {
            graphic_objects: self
                .graphic_objects
                .iter()
                .map(|graphic_object| graphic_object.zoom(k))
                .collect(),
        }
    }

    pub fn extend(&mut self, other: GraphicObjects) {
        self.graphic_objects.extend(other.graphic_objects);
    }

    pub fn from_strs(strings: Vec<&str>) -> GraphicObjects {
        let mut graphic_objects = GraphicObjects {
            graphic_objects: Vec::new(),
        };
        for line in strings.iter() {
            let splited = line.split_whitespace().collect::<Vec<&str>>();
            match splited[0] {
                "l" => graphic_objects
                    .graphic_objects
                    .push(Box::new(LineSegs2f::from_floats(
                        splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail"))
                            .collect(),
                    ))),
                "p" => graphic_objects
                    .graphic_objects
                    .push(Box::new(Polygon2f::from_floats(
                        splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail"))
                            .collect(),
                    ))),
                _ => panic!("Format error"),
            }
        }
        graphic_objects
    }

    pub fn into_iter(self) -> GraphicObjectsIntoIter {
        GraphicObjectsIntoIter {
            graphic_objects: self,
        }
    }
}

pub struct GraphicObjectsIntoIter {
    graphic_objects: GraphicObjects,
}

impl Iterator for GraphicObjectsIntoIter {
    type Item = Box<dyn GraphicObject>;

    fn next(&mut self) -> Option<Box<dyn GraphicObject>> {
        self.graphic_objects.graphic_objects.pop()
    }
}
