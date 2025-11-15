use lcf::{enums::Trigger, raw::lmu::event::instruction::Instruction};

pub struct ParallelEraseLint;

impl super::Lint for ParallelEraseLint {
    fn name(&self) -> &'static str {
        "Laggy parallel events should be erased after running"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                if page.trigger != Trigger::Parallel {
                    continue;
                }

                let mut has_laggy_instruction = false;
                let mut is_erased = false;

                for command in &page.commands {
                    match command.instruction {
                        Instruction::PlayBGM { .. }
                        | Instruction::MovePicture { .. }
                        | Instruction::ShowPicture { .. } => {
                            has_laggy_instruction = true;
                        }
                        Instruction::EraseEvent { .. } => {
                            is_erased = true;
                        }
                        _ => (),
                    }
                }

                if has_laggy_instruction && !is_erased {
                    diagnostics.push(super::Diagnostic {
                        event: Some(
                            super::DiagnosticEvent::from(event).with_page(
                                crate::lints::DiagnosticPage::new_from_index(page_index),
                            ),
                        ),
                        level: crate::lints::DiagnosticLevel::Warning,
                        message: None,
                    });
                }
            }
        }

        diagnostics
    }
}
