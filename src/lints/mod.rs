pub const ALL: &[&'static dyn Lint] = &[
    &weather::WeatherLint,
    &tissues::TissueLint,
    &v44_assignment::V44AssignmentLint,
    &instant_scroll::InstantScrollLint,
    &special_skills::SpecialSkillsLint,
    &comment::CommentLint,
    &show_picture::ShowPictureLint,
    &blue_sign::BlueSignLint,
    &pade_transfer::PadeTransferLint,
];

mod blue_sign;
mod comment;
mod instant_scroll;
mod pade_transfer;
mod show_picture;
mod special_skills;
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

impl Diagnostic {
    pub fn to_warning(self) -> Self {
        match self {
            Diagnostic::Normal => Diagnostic::Normal,
            Diagnostic::Warning(x) | Diagnostic::Error(x) => Diagnostic::Warning(x),
        }
    }
}

impl From<&[(&lcf::lmu::event::Event, usize, &str)]> for Diagnostic {
    fn from(value: &[(&lcf::lmu::event::Event, usize, &str)]) -> Self {
        if value.is_empty() {
            Diagnostic::Normal
        } else {
            Diagnostic::Error(
                value
                    .into_iter()
                    .map(|(event, page, msg)| {
                        format!(
                            "\n    EV{:04} (X{:03}, Y{:03}) on page {page}: {msg}",
                            event.id, event.x, event.y
                        )
                    })
                    .collect(),
            )
        }
    }
}
