use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct InstantScrollLint;

impl super::Lint for InstantScrollLint {
    fn name(&self) -> &'static str {
        "CEV0294 should be used for instant scroll"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut failures = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
                    match command.instruction {
                        Instruction::ScrollMap { speed, .. } => {
                            if speed == 53 {
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
                            "\n    EV{:04} (X{:03}, Y{:03}) on page {page} manually scrolls the map with speed 53",
                            event.id, event.x, event.y
                        )
                    })
                    .collect(),
            )
        }
    }
}
