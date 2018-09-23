extern crate rand;

use std::io::{self, Read};
use rand::prelude::*;
use std::collections::HashMap;

type Position = (i32, i32);
type Dimension = (i32, i32);

pub struct World {
    dimension: Dimension,
    cells: HashMap<Position, bool>
}

impl World {

    pub fn new(dimension: Dimension) -> World {
        let mut rng = thread_rng();
        let (width, height) = dimension;
        let mut m = HashMap::new();
        for x in 0..width {
            for y in 0..height {
                m.insert((x, y), rng.gen_bool(0.2));
            }
        }
        World { dimension: dimension, cells: m }
    }

    fn neighbours(&self, position: Position) -> u8 {
        let (x, y) = position;
        let candidates = vec![
            (x - 1, y),
            (x + 1, y),
            (x, y - 1),
            (x, y + 1),
            (x - 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y - 1),
            (x + 1, y + 1)
        ];
        candidates.iter()
            .map(|p| self.cells.get(&p))
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .filter(|c| **c)
            .collect::<Vec<_>>()
            .len() as u8
    }

    fn evolve_cell(&mut self, position: Position) {
        let n = self.neighbours(position);
        let state = if n < 2 || n > 3 {
            false
        } else if n == 3 {
            true
        } else {
            self.cells[&position]
        };
        self.cells.insert(position, state);
    }

    pub fn evolve(&mut self) {
        let (w, h) = self.dimension;
        for x in 0..w {
            for y in 0..h {
                self.evolve_cell((x, y));
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        let (w, h) = self.dimension;
        for x in 0..w {
            for y in 0..h {
                s.push_str(if self.cells[&(x, y)] {"*"} else {"."});
            }
            s.push_str("\n");
        }
        s
    }
}

fn main() {
    let mut world = World::new((10, 20));
    for _c in io::stdin().bytes() {
        println!("{}", world.to_string());
        world.evolve();
    }
}
