use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct SpecialSkillsLint;

impl super::Lint for SpecialSkillsLint {
    fn name(&self) -> &'static str {
        "Special skill usage must be annotated"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut excused = false;
        let mut failures = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
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
            Diagnostic::Error(failures
                .into_iter()
                .map(|(event, page)| {
                    format!(
                        "\n    EV{:04} (X{:03}, Y{:03}) on page {page:02}: Special skill used to check map completion without annotation",
                        event.id, event.x, event.y
                    )
                })
                .collect())
        }
    }
}
