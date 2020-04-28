use crate::algebra::Point2f;

#[derive(Clone, Debug)]
pub struct LineSegs2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4], // rgba
}
impl LineSegs2f {
    pub fn new(vertices: Vec<Point2f>, color: [f32; 4]) -> LineSegs2f {
        LineSegs2f {
            vertices: vertices,
            color: color,
        }
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

#[derive(Clone, Debug)]
pub struct Polygon2f {
    pub vertices: Vec<Point2f>,
    pub color: [f32; 4],
}

#[derive(Clone, Debug)]
pub enum GraphicObject {
    LineSegs(LineSegs2f),
    Polygon(Polygon2f),
}

impl GraphicObject {
    pub fn shift(&self, dp: Point2f) -> GraphicObject {
        match self {
            GraphicObject::LineSegs(line_segs) => {
                let mut result: LineSegs2f = line_segs.clone();
                for vertex in &mut result.vertices {
                    *vertex += dp;
                }
                GraphicObject::LineSegs(result)
            }
            GraphicObject::Polygon(polygon) => {
                unimplemented!();
            }
        }
    }
}

#[derive(Clone)]
pub struct GraphicObjects {
    graphic_objects: Vec<GraphicObject>,
}

impl GraphicObjects {
    pub fn from_strs(strings: Vec<&str>) -> GraphicObjects {
        let mut graphic_objects = GraphicObjects {
            graphic_objects: Vec::new(),
        };
        for line in strings.iter() {
            let splited = line.split_whitespace().collect::<Vec<&str>>();
            match splited[0] {
                "l" => graphic_objects
                    .graphic_objects
                    .push(GraphicObject::LineSegs(LineSegs2f::from_floats(
                        splited[1..]
                            .iter()
                            .map(|x| x.parse::<f32>().expect("float parse fail"))
                            .collect(),
                    ))),
                "p" => unimplemented!(),
                _ => panic!("Format error"),
            }
        }
        graphic_objects
    }

    pub fn get(&self, id: usize) -> Option<&GraphicObject> {
        self.graphic_objects.get(id)
    }
}

#[cfg(test)]
mod test {
    use super::Point2f;
    use super::{GraphicObject, GraphicObjects};

    #[test]
    fn graphic_objects_from_strs() {
        let graphic_objects =
            GraphicObjects::from_strs(vec!["l 0.3 1.0 1.0 1.0 -2 -2 -2 2 2 2 2 -2"]);
        match graphic_objects.get(0) {
            None => panic!("test failed"),
            Some(graphic_object) => match graphic_object {
                GraphicObject::Polygon(_) => panic!("test failed"),
                GraphicObject::LineSegs(polygon) => {
                    assert_eq!(polygon.vertices[0], Point2f::from_floats(-2., -2.));
                    assert!(polygon.vertices.get(4).is_none());
                }
            },
        }
        assert!(graphic_objects.get(1).is_none());
    }
}
