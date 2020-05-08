use std::fs::File;
use std::io::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Record {
    // e.g. dt_seq 1. 1. 1.
    // times 1. 2. 3.
    // operations[0] is executed in times = 1.
    // so when an operation is recorded, that frame's tick() has not been called
    pub params: (u64, f32, f32, f32, f32),
    pub dt_seq: Vec<f32>,
    pub operation: Vec<(usize, i8, bool)>, //frame, key_id, updown
}

impl Record {
    pub fn save(&self, filename: String) {
        let mut file = File::create(&filename).unwrap();
        file.write_all(&bincode::serialize(self).unwrap()).unwrap();
    }

    pub fn load(filename: String) -> Record {
        let mut file = File::open(&filename).unwrap();
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer).unwrap();
        bincode::deserialize(&buffer).unwrap()
    }
}
