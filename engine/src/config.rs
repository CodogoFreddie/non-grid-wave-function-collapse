pub enum BrushFalloff {
    No, // plain solid circle
    Circular, // sqrt fall off
    Linear,
}

pub struct BrushConfig {
    width: f64, // width of shape that brush paints
    falloff: BrushFalloff, // type of falloff
    max_stipple: f64, // maximum distance between paints
    min_stipple: f64, // minimum distance between paints
}

pub struct SourceSampleConfig {
    max_distance: f64, // the max distance apart that two samples of the input ruleset can be
    min_distance: f64, // the min distance apart that two samples of the input ruleset can be
    sample_attempts: u64, // the number of attempts to find a valid neighbour point before a point is considered boxed-in
}

pub struct Rule {
    radius_of_points: f64, // distance between samples and center
    number_of_points: u8, // number of equally distant radial samples to take from a central point to decide what rules to apply to it.
}

pub struct Color(r: i8, g: i8, b: i8);

pub struct State {
    color: Color, 
    index,
}

pub struct Config {
    brush: BrushConfig,
    running_samplling: RunningSampleConfig,
    source_sampling: SourceSampleConfig,
    states: Vec<State>,
}
