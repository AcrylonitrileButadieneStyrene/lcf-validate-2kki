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
                                failures.push((
                                    event,
                                    page_index + 1,
                                    "manually scrolls with speed 53",
                                ))
                            }
                        }

                        _ => (),
                    }
                }
            }
        }

        Diagnostic::from(failures.as_ref())
    }
}
