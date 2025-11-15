use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct PadeTransferLint;

impl super::Lint for PadeTransferLint {
    fn name(&self) -> &'static str {
        "Transitioning maps should be unPADEed"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut failures = Vec::new();

        let exclusions = vec!["移動先マップで直接「ｲﾍﾞﾝﾄ中動作禁止解除」しています。"];

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                let mut pade = false;
                let mut moved = false;
                let mut ignored = false;

                for command in &page.commands {
                    match command.instruction {
                        Instruction::CallEvent { mode, index, .. } if mode == 0 && index == 8 => {
                            // PADE
                            pade = true;
                        }
                        Instruction::CallEvent { mode, index, .. } if mode == 0 && index == 9 => {
                            // unPADE
                            pade = false;
                        }
                        Instruction::TransferPlayer { .. } if pade => {
                            moved = true;
                        }
                        Instruction::Comment | Instruction::CommentNextLine => {
                            let string = encoding_rs::SHIFT_JIS.decode(&command.string).0;
                            if !exclusions
                                .iter()
                                .any(|exclusion| string.contains(exclusion))
                            {
                                ignored = true;
                            }
                        }
                        _ => (),
                    }
                }

                if pade && moved && !ignored {
                    failures.push((event, page_index + 1, ""));
                }
            }
        }

        Diagnostic::from(failures.as_ref())
    }
}
