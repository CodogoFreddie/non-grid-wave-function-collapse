extern crate rand;

pub mod config;
mod point_set;

use crate::config::Config;

type TrainingData = [u8];

pub fn generate_rule_set<'a>(
    config: &'a Config,
    _training_data: &TrainingData,
) -> point_set::PointSet<'a> {
    // first, generate a set of points within the training data bounds using piosson disk sampling,
    //
    // then, use those points to generate a RuleSet

    let mut point_set = point_set::PointSet::new(&config);

    while let Some(p) = point_set.add_new_point() {}

    point_set
}
