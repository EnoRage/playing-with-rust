use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize,)]
pub struct people {
    name: String,
    family: String,
    age: i8,
    huy_size: i32,
}

pub fn people() -> people {
    people {
        name: String::from(""),
        family: String::from(""),
        age: 0,
        huy_size: 0,
    }
}

impl Iterator for people {
    type Item = people;

    fn next(&mut self) -> Option<people> {
        let mut rng = rand::thread_rng();
        let new_next = people {
            name: rng.gen::<(u32)>().to_string(),
            family: rng.gen::<u32>().to_string(),
            age: rng.gen(),
            huy_size: rng.gen(),
        };

        Some(new_next)
    }
}
