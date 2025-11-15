use lcf::raw::lmu::event::instruction::Instruction;

use crate::{
    Diagnostic,
    lints::{DiagnosticEvent, DiagnosticLevel},
};

pub struct BlueSignLint;

impl super::Lint for BlueSignLint {
    fn name(&self) -> &'static str {
        "Blue signs must have annotations"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<Diagnostic> {
        let exclusions = ["eserved", "予約", "接続"];
        let mut diagnostics = Vec::new();

        for event in &map.events {
            let is_blue_sign = event.pages.iter().any(|page| {
                encoding_rs::SHIFT_JIS.decode(&page.graphic.file).0 == "system_kyouyu_gazou06"
                    && (page.graphic.index == 1 || page.graphic.index == 2)
            });

            if is_blue_sign {
                let allowed = event
                    .pages
                    .iter()
                    .flat_map(|page| page.commands.iter())
                    .filter_map(|command| match command.instruction {
                        Instruction::Comment | Instruction::CommentNextLine => {
                            Some(command.string.clone())
                        }
                        _ => None,
                    })
                    .any(|comment| {
                        let comment = encoding_rs::SHIFT_JIS.decode(&comment).0;
                        exclusions
                            .iter()
                            .any(|exclusion| comment.contains(exclusion))
                    });

                if !allowed {
                    diagnostics.push(Diagnostic {
                        level: DiagnosticLevel::Error,
                        event: Some(DiagnosticEvent::from(event)),
                        message: None,
                    });
                }
            }
        }

        diagnostics
    }
}
