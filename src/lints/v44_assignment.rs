use lcf::raw::lmu::event::instruction::Instruction;

pub struct V44AssignmentLint;

impl super::Lint for V44AssignmentLint {
    fn name(&self) -> &'static str {
        "V0044 should not be assigned to"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for (command_index, command) in page.commands.iter().enumerate() {
                    if let Instruction::ControlVariables {
                        mode, start, end, ..
                    } = command.instruction
                        && ((mode == 0 && start == 44) || (mode == 1 && start <= 44 && 44 <= end))
                    {
                        diagnostics.push(super::Diagnostic {
                            event: Some(super::DiagnosticEvent::from(event).with_page(
                                super::DiagnosticPage::new_from_indexes(page_index, command_index),
                            )),
                            level: super::DiagnosticLevel::Error,
                            message: None,
                        });
                    }
                }
            }
        }

        diagnostics
    }
}
