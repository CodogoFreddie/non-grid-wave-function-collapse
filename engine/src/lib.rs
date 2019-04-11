
mod config;
use config::Config;

type Rule = Vec<u8>;
type RuleSet = Vec<Rule>;
type TrainingData = [u8];

struct Point {
    x: u16,
    y: u16,
}

fn generate_rule_centered_on_point(x: u16, y: u16, config: &Config, training_data: &TrainingData) -> Option<Rule> {
    Some( Rule::new() )
}

pub fn generate_rule_set(config: &Config, training_data: &TrainingData) -> RuleSet {
    let mut open_points: Vec<Point> = Vec::new();
    let mut closed_points: Vec<Point> = Vec::new();
    let mut rule_set: RuleSet = Vec::new();

    open_points.push( Point {
            x: config.source.width / 2,
            y: config.source.height / 2,
    });

    while open_points.len() > 0 {
        
    }

    rule_set
}
