use crate::algebra;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                for node in &mut result.nodes {
                    *node += dp;
                }
                GraphicObject::Polygen(result)
            }
        }
    }
}

pub type GraphicObjects = Vec<GraphicObject>;

impl GraphicObjects {
    pub fn load(&mut self, filename: &str) {  
        let file = File::open(filename).unwrap();
        for line in io::BufReader::new(file).lines() {
            splited = line.split_whitespace().collect::<Vec<&str>>();
            match splited[0] {
                "p" => {
                    self.append(GraphicObject::Polygen(
                        algebra::Polygen2f::from_floats(splited[1:])
                    ))
                }
                "l" => {
                    unimplemented!()
                }
                _ => panic!("Format error"),
            }
        }
    }
}
