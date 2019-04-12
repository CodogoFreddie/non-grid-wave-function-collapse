use rayon::prelude::*;

use rand::Rng;
use std::collections::HashSet;

use crate::config::Config;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Point {
    x: u16,
    y: u16,
}

pub struct PointSet<'a> {
    open_points: HashSet<Point>,
    closed_points: HashSet<Point>,
    config: &'a Config,
}

impl<'a> PointSet<'a> {
    pub fn new(config: &'a Config) -> PointSet {
        PointSet {
            closed_points: HashSet::new(),
            config,
            open_points: HashSet::new(),
        }
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
        for _ in 0..self.config.source.sampling.attempts {
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
        let r = rng.gen_range(
            self.config.source.sampling.min_distance,
            self.config.source.sampling.max_distance,
        );

        return Point {
            x: (start_point.x + ((r * f64::cos(theta)) as u16)),
            y: (start_point.y + ((r * f64::sin(theta)) as u16)),
        };
    }

    fn point_is_inside_bounds(&self, point: &Point) -> bool {
        point.x > 0 + self.config.rules.radius_of_points as u16
            || point.y > 0 + self.config.rules.radius_of_points as u16
            || point.x < self.config.source.width + self.config.rules.radius_of_points as u16
            || point.y < self.config.source.height + self.config.rules.radius_of_points as u16
    }

    fn point_is_far_to_exising_points(&self, point: &Point) -> bool {
        let comparator = |existing_point: &Point| {
            let dx = existing_point.x - point.x;
            let dy = existing_point.y - point.y;

            let d = f64::sqrt((dx * dx) as f64 + (dy * dy) as f64);

            d > self.config.source.sampling.min_distance
        };

        self.open_points.par_iter().all(&comparator)
    }
}

