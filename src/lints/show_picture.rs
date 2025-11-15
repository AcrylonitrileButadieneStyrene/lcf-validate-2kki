use lcf::raw::lmu::event::instruction::Instruction;

pub struct ShowPictureLint;

impl super::Lint for ShowPictureLint {
    // "Also, although the behavior of 'Show Picture' looks the same as 'Move Picture,' it is preferable to use 'Move Picture,' as it is lighter in terms of processing load."
    fn name(&self) -> &'static str {
        "MovePicture is preferrable to ShowPicture"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for (command_index, command) in page.commands.iter().enumerate() {
                    if let Instruction::ShowPicture { .. } = command.instruction {
                        diagnostics.push(super::Diagnostic {
                            event: Some(super::DiagnosticEvent::from(event).with_page(
                                super::DiagnosticPage::new_from_indexes(page_index, command_index),
                            )),
                            level: super::DiagnosticLevel::Warning,
                            message: None,
                        });
                    }
                }
            }
        }

        diagnostics
    }
}
