use lcf::raw::lmu::event::instruction::Instruction;

pub struct PadeTransferLint;

impl super::Lint for PadeTransferLint {
    fn name(&self) -> &'static str {
        "Transitioning maps should be unPADEed"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut diagnostics = Vec::new();

        let exclusions = ["移動先マップで直接「ｲﾍﾞﾝﾄ中動作禁止解除」しています。"];

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
                    diagnostics.push(super::Diagnostic {
                        event: Some(
                            super::DiagnosticEvent::from(event).with_page(
                                crate::lints::DiagnosticPage::new_from_index(page_index),
                            ),
                        ),
                        level: crate::lints::DiagnosticLevel::Error,
                        message: None,
                    });
                }
            }
        }

        diagnostics
    }
}
