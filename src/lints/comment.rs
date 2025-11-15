use lcf::raw::lmu::event::instruction::Instruction;

pub struct CommentLint;

impl super::Lint for CommentLint {
    fn name(&self) -> &'static str {
        "Comments should not be too long"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for (command_index, command) in page.commands.iter().enumerate() {
                    match command.instruction {
                        Instruction::Comment | Instruction::CommentNextLine => {
                            let max = 56u32.saturating_sub(command.indent * 2);
                            let len = encoding_rs::SHIFT_JIS
                                .decode(&command.string)
                                .0
                                .chars()
                                .count();
                            if len > max as usize {
                                diagnostics.push(super::Diagnostic {
                                    event: Some(super::DiagnosticEvent::from(event).with_page(
                                        super::DiagnosticPage::new_from_indexes(
                                            page_index,
                                            command_index,
                                        ),
                                    )),
                                    level: super::DiagnosticLevel::Warning,
                                    message: Some(format!("{len}/{max}")),
                                });
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        diagnostics
    }
}
