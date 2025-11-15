use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct CommentLint;

impl super::Lint for CommentLint {
    fn name(&self) -> &'static str {
        "Comments should not be too long"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut failures = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
                    match command.instruction {
                        Instruction::Comment | Instruction::CommentNextLine => {
                            let max = 56u32.saturating_sub(command.indent * 2);
                            let len = encoding_rs::SHIFT_JIS
                                .decode(&command.string)
                                .0
                                .chars()
                                .count();
                            if len > max as usize {
                                failures.push((event, page_index + 1, "comment too long"));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        Diagnostic::from(failures.as_ref()).to_warning()
    }
}
