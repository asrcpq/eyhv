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

    fn render(&self, canvas: &mut Canvas) {
        for vertice in self.vertices.iter() {
            if !WINDOW_RECT.contain(*vertice) {
                continue;
            }
            let location = canvas.map_point2f(*vertice);

            canvas.data[location] = self.color[0] as u8 * 255;
            canvas.data[location + 1] = self.color[1] as u8 * 255;
            canvas.data[location + 2] = self.color[2] as u8 * 255;
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
