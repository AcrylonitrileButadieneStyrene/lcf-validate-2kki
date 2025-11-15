use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct ShowPictureLint;

impl super::Lint for ShowPictureLint {
    // "Also, although the behavior of 'Show Picture' looks the same as 'Move Picture,' it is preferable to use 'Move Picture,' as it is lighter in terms of processing load."
    fn name(&self) -> &'static str {
        "MovePicture is preferrable to ShowPicture"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut failures = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
                    match command.instruction {
                        Instruction::ShowPicture { .. } => {
                            failures.push((event, page_index + 1, ""));
                        }
                        _ => (),
                    }
                }
            }
        }

        Diagnostic::from(failures.as_ref()).to_warning()
    }
}
