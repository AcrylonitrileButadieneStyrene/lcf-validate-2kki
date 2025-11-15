use std::num::NonZeroU32;

use lcf::lmu::event::Event;

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
    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<Diagnostic>;
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub event: Option<DiagnosticEvent>,
    pub message: Option<String>,
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(event) = &self.event {
            write!(f, "EV{:04} (X{:03}, Y{:03})", event.id, event.x, event.y)?;
            if let Some(page) = &event.page {
                write!(f, " P{:02}", page.id)?;
                if let Some(command) = page.command {
                    write!(f, " I{command:05}")?;
                }
            }

            if let Some(message) = &self.message {
                write!(f, ": {message}")?;
            }
        } else if let Some(message) = &self.message {
            f.write_str(message)?;
        } else {
            f.write_str("<No information provided>")?;
        }

        Ok(())
    }
}

impl From<Diagnostic> for Vec<Diagnostic> {
    fn from(value: Diagnostic) -> Self {
        vec![value]
    }
}

pub enum DiagnosticLevel {
    Warning,
    Error,
}

pub struct DiagnosticEvent {
    id: NonZeroU32,
    x: u32,
    y: u32,
    page: Option<DiagnosticPage>,
}

impl DiagnosticEvent {
    const fn with_page(mut self, page: DiagnosticPage) -> Self {
        self.page = Some(page);
        self
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&Event> for DiagnosticEvent {
    fn from(value: &Event) -> Self {
        Self {
            id: NonZeroU32::new(value.id).unwrap(),
            x: value.x,
            y: value.y,
            page: None,
        }
    }
}

pub struct DiagnosticPage {
    id: NonZeroU32,
    command: Option<NonZeroU32>,
}

impl DiagnosticPage {
    #[must_use]
    pub const fn new_from_index(page_index: usize) -> Self {
        Self {
            id: NonZeroU32::new(page_index as u32 + 1).unwrap(),
            command: None,
        }
    }

    #[must_use]
    pub const fn new_from_indexes(page_index: usize, command_index: usize) -> Self {
        Self {
            id: NonZeroU32::new(page_index as u32 + 1).unwrap(),
            command: Some(NonZeroU32::new(command_index as u32 + 1).unwrap()),
        }
    }
}
