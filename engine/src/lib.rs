use std::vec::Vec;

enum Terrain {
    Grass,
    Water,
    Rock,
}

struct State {
    grass: f32,
    water: f32,
    rock: f32,
}

struct World {
    points: Vec<State>,
    falloff: usize,
}

const SIZE: usize = 512;
impl World {
    pub fn new() -> Self {
        World {
            points: Vec::with_capacity(SIZE * SIZE),
            falloff: 8,
        }
    }

    fn get_point(&self, x: usize, y: usize) -> Option<&State> {
        self.points.get((y * SIZE) + x)
    }

    fn reduce_value_arround_point(&self, terrain_type: &Terrain) -> () {
        for x in 0..self.falloff {
            let x_pos = x % SIZE;
            let x_neg = x_pos;
            for y in 0..self.falloff {
                let y_pos = y % SIZE;
                let y_neg = y_pos;

                let dist
            }
        }

        ()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
