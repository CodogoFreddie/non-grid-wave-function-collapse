use newfc_engine::config::Config;
use newfc_engine::generate_rule_set;

fn open_or_create_config() -> Config {
    use newfc_engine::config::*;
    use std::env;
    use std::fs;

    match fs::read_to_string("./config.json") {
        Err(_) => {
            let config = Config {
                brush: BrushConfig {
                    width: 10.0,
                    falloff: BrushFalloff::Circular,
                    max_stipple: 6.0,
                    min_stipple: 4.0,
                },
                source: SourceConfig {
                    sampling: SourceSampleConfig {
                        max_distance: 10.0,
                        min_distance: 5.0,
                        attempts: 256,
                    },
                    width: 256,
                    height: 256,
                },
                states: vec![StateConfig {
                    color: Color {
                        r: 250,
                        g: 100,
                        b: 100,
                    },
                }],
                rules: RuleConfig {
                    radius_of_points: 10.0,
                    number_of_points: 5,
                },
            };

            fs::write(
                "./config.json",
                serde_json::to_string_pretty(&config).unwrap(),
            );

            config
        }

        Ok(string) => {
            let config: Config = serde_json::from_str(&string).unwrap();

            config
        }
    }
}

fn main() {
    let config = open_or_create_config();

    let rule_set = generate_rule_set(&config, &[]);

    for point in rule_set.points() {
        println!("{:?}", point);
    }
}
