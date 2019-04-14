use rayon::prelude::*;

use rand::Rng;
use std::collections::HashSet;

use crate::config::Config;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Point {
    x: u16,
    y: u16,
}

#[derive(Debug)]
pub struct FishDiskPoints {
    attempts: u16,
    distance_max: f64,
    distance_min: f64,
    height: u16,
    pub closed_points: HashSet<Point>,
    pub open_points: HashSet<Point>,
    width: u16,
}

impl FishDiskPoints {
    pub fn new(
        width: u16,
        height: u16,
        distance_min: f64,
        distance_max: f64,
        attempts: u16,
    ) -> FishDiskPoints {
        let mut me = FishDiskPoints {
            attempts,
            closed_points: HashSet::new(),
            distance_max,
            distance_min,
            height,
            open_points: HashSet::new(),
            width,
        };

        me.open_points.insert(
            me.get_neighbour_of_start_point(&Point {
                x: width / 2,
                y: height / 2,
            })
            .unwrap(),
        );

        me
    }

    pub fn points(&self) -> std::collections::hash_set::Iter<'_, Point> {
        self.closed_points.iter()
    }

    /// recursivly searches all open_points and trys to add a new point that is not too close and
    /// not too far from the existing points
    pub fn add_new_point(&mut self) -> Option<&Point> {
        let start_point = match self.open_points.iter().cloned().nth(0) {
            None => return None,
            Some(x) => x,
        };

        match self.get_neighbour_of_start_point(&start_point) {
            None => {
                if let Some(retired_point) = self.open_points.take(&start_point) {
                    self.closed_points.insert(retired_point);
                }

                self.add_new_point()
            }

            Some(valid_neighbour) => {
                self.open_points.insert(valid_neighbour.clone());

                Some(self.open_points.get(&valid_neighbour).unwrap())
            }
        }
    }

    /// given a start_point, makes attempts to find a neighbouring point that is within the config
    /// bounds and not too close to an existing point
    fn get_neighbour_of_start_point(&self, start_point: &Point) -> Option<Point> {
        for _ in 0..self.attempts {
            let candidate_point = self.generate_candidate_point(&start_point);

            if self.point_is_inside_bounds(&candidate_point)
                && self.point_is_far_to_exising_points(&candidate_point)
            {
                return Some(candidate_point);
            }
        }

        None
    }

    fn generate_candidate_point(&self, start_point: &Point) -> Point {
        let mut rng = rand::thread_rng();

        let theta = rng.gen_range(0.0, 3.1415 * 2.0);
        let r = rng.gen_range(self.distance_min, self.distance_max);

        return Point {
            x: (start_point.x as i16 + ((r * f64::cos(theta)) as i16)) as u16,
            y: (start_point.y as i16 + ((r * f64::sin(theta)) as i16)) as u16,
        };
    }

    fn point_is_inside_bounds(&self, point: &Point) -> bool {
        point.x > 0 + self.distance_max as u16
            && point.y > 0 + self.distance_max as u16
            && point.x < self.width - self.distance_max as u16
            && point.y < self.height - self.distance_max as u16
    }

    fn point_is_far_to_exising_points(&self, point: &Point) -> bool {
        let comparator = |existing_point: &Point| {
            let dx = existing_point.x as i64 - point.x as i64;
            let dy = existing_point.y as i64 - point.y as i64;

            let d = f64::sqrt((dx * dx) as f64 + (dy * dy) as f64);

            d > self.distance_min
        };

        self.open_points.par_iter().all(&comparator)
            && self.closed_points.par_iter().all(&comparator)
    }
}
