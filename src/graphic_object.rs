use crate::algebra;

use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub enum GraphicObject {
    LineSegs(algebra::LineSegs2f),
    Polygon(algebra::Polygon2f),
}

impl GraphicObject {
    pub fn shift(&self, dp: algebra::Point2f) -> GraphicObject {
        match self {
            GraphicObject::LineSegs(line_segs) => {
                let mut result: algebra::LineSegs2f = line_segs.clone();
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

pub struct GraphicObjects {
    graphic_objects: Vec<GraphicObject>,
}

impl GraphicObjects {
    pub fn from_path(filename: String) -> GraphicObjects {
        println!("GraphicObjects loading from: {}", filename);
        let mut graphic_objects = GraphicObjects {
            graphic_objects: Vec::new(),
        };
        let file = File::open(filename).unwrap();
        for line_result in io::BufReader::new(file).lines() {
            if let Ok(line) = line_result {
                let splited = line.split_whitespace().collect::<Vec<&str>>();
                match splited[0] {
                    "l" => graphic_objects
                        .graphic_objects
                        .push(GraphicObject::LineSegs(algebra::LineSegs2f::from_floats(
                            splited[1..]
                                .iter()
                                .map(|x| x.parse::<f32>().expect("float parse fail"))
                                .collect(),
                        ))),
                    "p" => unimplemented!(),
                    _ => panic!("Format error"),
                }
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
    use super::algebra;
    use super::{GraphicObject, GraphicObjects};

    #[test]
    fn graphic_objects_load() {
        std::fs::write("/tmp/graphic_objects_load_test", "l -2 -2 -2 2 2 2 2 -2")
            .expect("unable to write file");
        let graphic_objects =
            GraphicObjects::from_path("/tmp/graphic_objects_load_test".to_string());
        match graphic_objects.get(0) {
            None => panic!("test failed"),
            Some(graphic_object) => match graphic_object {
                GraphicObject::Polygon(_) => panic!("test failed"),
                GraphicObject::LineSegs(polygon) => {
                    assert_eq!(polygon.vertices[0], algebra::Point2f::from_floats(-2., -2.));
                    assert!(polygon.vertices.get(4).is_none());
                }
            },
        }
        assert!(graphic_objects.get(1).is_none());
        std::fs::remove_file("/tmp/graphic_objects_load_test").unwrap();
    }
}
