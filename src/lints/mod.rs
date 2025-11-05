pub const ALL: [&'static dyn Lint; 1] = [&weather::WeatherLint];

mod weather;

pub trait Lint {
    fn name(&self) -> &'static str;
    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Result<(), String>;
}
