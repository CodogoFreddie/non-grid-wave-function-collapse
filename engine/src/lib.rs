extern crate rand;

mod config;
mod point_set;

use crate::config::Config;

type TrainingData = [u8];

pub fn generate_rule_set(config: &Config, _training_data: &TrainingData) -> () {
    // first, generate a set of points within the training data bounds using piosson disk sampling,
    //
    // then, use those points to generate a RuleSet

    let mut point_set = point_set::PointSet::new(&config);

    while let Some(_) = point_set.add_new_point() {

    }
}
