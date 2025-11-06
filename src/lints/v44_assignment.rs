use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct V44AssignmentLint;

impl super::Lint for V44AssignmentLint {
    fn name(&self) -> &'static str {
        "V0044 should not be assigned to"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut failures = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
                    match command.instruction {
                        Instruction::ControlVariables {
                            mode, start, end, ..
                        } => {
                            if (mode == 0 && start == 44) || (mode == 1 && start <= 44 && 44 <= end)
                            {
                                failures.push((event, page_index + 1))
                            }
                        }

                        _ => (),
                    }
                }
            }
        }

        if failures.is_empty() {
            Diagnostic::Normal
        } else {
            Diagnostic::Error(
                failures
                    .into_iter()
                    .map(|(event, page)| {
                        format!(
                            "\n    EV{:04} (X{:03}, Y{:03}) on page {page} writes to V0044",
                            event.id, event.x, event.y
                        )
                    })
                    .collect(),
            )
        }
    }
}
