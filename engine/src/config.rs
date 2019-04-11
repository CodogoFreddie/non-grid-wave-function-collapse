pub enum BrushFalloff {
    No, // plain solid circle
    Circular, // sqrt fall off
    Linear,
}

pub struct BrushConfig {
    pub width: f64, // width of shape that brush paints
    pub falloff: BrushFalloff, // type of falloff
    pub max_stipple: f64, // maximum distance between paints
    pub min_stipple: f64, // minimum distance between paints
}

pub struct SourceSampleConfig {
    pub max_distance: f64, // the max distance apart that two samples of the input ruleset can be
    pub min_distance: f64, // the min distance apart that two samples of the input ruleset can be
    pub sample_attempts: u64, // the number of attempts to find a valid neighbour point before a point is considered boxed-in
}

pub struct SourceConfig {
    pub sampling: SourceSampleConfig,
    pub width: u16,
    pub height: u16,
}

pub struct RuleConfig {
    pub radius_of_points: f64, // distance between samples and center
    pub number_of_points: u8, // number of equally distant radial samples to take from a central point to decide what rules to apply to it.
}

pub struct Color{r: i8, g: i8, b: i8}

pub struct StateConfig {
    pub color: Color, 
    pub index: u8,
}

pub struct RenderingConfig {
    pub rerender_from_scratch_on_state_pop: bool,
}

pub struct Config {
    pub brush: BrushConfig,
    pub rendering: RenderingConfig,
    pub source: SourceConfig,
    pub states: Vec<StateConfig>,
    pub rules: RuleConfig,
}
