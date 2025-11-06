pub const ALL: &[&'static dyn Lint] = &[
    &weather::WeatherLint,
    &tissues::TissueLint,
    &v44_assignment::V44AssignmentLint,
];

mod tissues;
mod v44_assignment;
mod weather;

pub trait Lint {
    fn name(&self) -> &'static str;
    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic;
}

pub enum Diagnostic {
    Normal,
    Warning(String),
    Error(String),
}
