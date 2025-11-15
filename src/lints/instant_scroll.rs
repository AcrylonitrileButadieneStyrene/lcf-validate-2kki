use lcf::raw::lmu::event::instruction::Instruction;

pub struct InstantScrollLint;

impl super::Lint for InstantScrollLint {
    fn name(&self) -> &'static str {
        "CEV0294 should be used for instant scroll"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for (command_index, command) in page.commands.iter().enumerate() {
                    if let Instruction::ScrollMap { speed, .. } = command.instruction
                        && speed == 53
                    {
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
