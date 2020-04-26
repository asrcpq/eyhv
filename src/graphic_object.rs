use crate::algebra;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
pub enum GraphicObject {
    Line(algebra::LineSeg2f),
    Polygen(algebra::Polygen2f),
}

impl GraphicObject {
    pub fn shift(&self, dp: algebra::Point2f) -> GraphicObject {
        match self {
            GraphicObject::Line(line) => {
                let mut result: algebra::LineSeg2f = *line;
                result.begin += dp;
                result.end += dp;
                GraphicObject::Line(result)
            }
            GraphicObject::Polygen(polygen) => {
                let mut result: algebra::Polygen2f = polygen.clone();
                for vertex in &mut result.vertices {
                    *vertex += dp;
                }
                GraphicObject::Polygen(result)
            }
        }
    }
}

pub struct GraphicObjects {
    graphic_objects: Vec<GraphicObject>,
}

impl GraphicObjects {
    pub fn from_path(filename: &str) -> GraphicObjects {  
        let mut graphic_objects = GraphicObjects {
            graphic_objects: Vec::new(),
        };
        let file = File::open(filename).unwrap();
        for line_result in io::BufReader::new(file).lines() {
            if let Ok(line) = line_result {
                let splited = line.split_whitespace().collect::<Vec<&str>>();
                match splited[0] {
                    "p" => {
                        graphic_objects.graphic_objects.push(GraphicObject::Polygen(
                            algebra::Polygen2f::from_floats(
                                splited[1 ..]
                                .iter()
                                .map(|x| x.parse::<f32>().expect("float parse fail"))
                                .collect()
                            )
                        ))
                    }
                    "l" => {
                        unimplemented!()
                    }
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
    use super::{GraphicObjects, GraphicObject};
    use super::algebra;

    #[test]
    fn graphic_objects_load() {
        std::fs::write("/tmp/graphic_objects_load_test", "p -2 -2 -2 2 2 2 2 -2")
            .expect("unable to write file");
        let graphic_objects = GraphicObjects::from_path("/tmp/graphic_objects_load_test");
        match graphic_objects.get(0) {
            None => panic!("test failed"),
            Some(graphic_object) => {
                match graphic_object {
                    GraphicObject::Line(_) => panic!("test failed"),
                    GraphicObject::Polygen(polygen) => {
                        assert_eq!(
                            polygen.vertices[0],
                            algebra::Point2f::from_floats(-2., -2.)
                        );
                        assert!(polygen.vertices.get(4).is_none());
                    },
                }
            }
        }
        assert!(graphic_objects.get(1).is_none());
        std::fs::remove_file("/tmp/graphic_objects_load_test").unwrap();
    }
}
