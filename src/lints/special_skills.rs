use lcf::raw::lmu::event::instruction::Instruction;

pub struct SpecialSkillsLint;

impl super::Lint for SpecialSkillsLint {
    fn name(&self) -> &'static str {
        "Special skill usage must be annotated"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut excused = false;
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for (command_index, command) in page.commands.iter().enumerate() {
                    match command.instruction {
                        Instruction::Comment | Instruction::CommentNextLine => {
                            // TODO: in the future, this should validate that the skill it says
                            //       it is checking is the same one that it is actually checking.
                            if encoding_rs::SHIFT_JIS
                                .decode(&command.string)
                                .0
                                .contains("▽Skills")
                            {
                                excused = true;
                            }
                        }
                        Instruction::ConditionalBranch {
                            mode,   // 5 == actor
                            field1, // 2 == map completion (2kki specific)
                            field2, // 4 == knows skill
                            ..
                        } if mode == 5 && field1 == 2 && field2 == 4 => {
                            if !excused {
                                diagnostics.push(super::Diagnostic {
                                    event: Some(super::DiagnosticEvent::from(event).with_page(
                                        super::DiagnosticPage::new_from_indexes(
                                            page_index,
                                            command_index,
                                        ),
                                    )),
                                    level: super::DiagnosticLevel::Error,
                                    message: None,
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
