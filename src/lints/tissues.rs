use lcf::raw::lmu::event::instruction::Instruction;

pub struct TissueLint;

impl super::Lint for TissueLint {
    fn name(&self) -> &'static str {
        "Tissue event validity"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let main_sig = encoding_rs::SHIFT_JIS.encode("ティッシュ++++").0.to_vec();
        let helper_sigs = vec![
            "ティッシ", // carciniara beach (Map1024.lmu) uses ゥ instead of ュ
            "tis",      // #fluxide uses this
                        // rustfmt doesn't understand that japanese characters are longer than latin ones
        ]
        .into_iter()
        .map(|valid| encoding_rs::SHIFT_JIS.encode(valid).0.to_vec())
        .collect::<Vec<_>>();

        let Some(tissue) = map.events.iter().find(|event| event.name == main_sig) else {
            return super::Diagnostic {
                event: None,
                level: super::DiagnosticLevel::Warning,
                message: Some("Does not have tissue events".to_string()),
            }
            .into();
        };

        let helpers = tissue
            .pages
            .iter()
            .flat_map(|page| {
                // sugar road puts the main code on the 2nd page instead of the 1st
                page.commands
                    .iter()
                    .filter_map(|command| match &command.instruction {
                        Instruction::CallEvent { index, .. } => Some(*index),
                        // the dream balcony manually moves them
                        Instruction::SetEventLocation { source, .. } => Some(*source),
                        _ => None,
                    })
            })
            .collect::<Vec<_>>();
        if helpers.len() != 5 {
            return super::Diagnostic {
                event: Some(super::DiagnosticEvent::from(tissue)),
                level: super::DiagnosticLevel::Warning,
                message: Some(format!(
                    "Expected 5 tissues but found {}. This is likely a bug with this tool.",
                    helpers.len()
                )),
            }
            .into();
        }

        helpers
            .iter()
            .enumerate()
            .filter_map(|(index, id)| {
                let Some(event) = map.events.iter().find(|event| event.id == *id) else {
                    return Some(super::Diagnostic {
                        event: None,
                        level: super::DiagnosticLevel::Error,
                        message: Some(format!(
                            "Tissue {} points to non-existent event EV{id:04}",
                            index + 1
                        )),
                    });
                };

                if !helper_sigs.iter().any(|sig| event.name.starts_with(sig)) {
                    return Some(super::Diagnostic {
                        event: Some(super::DiagnosticEvent::from(event)),
                        level: super::DiagnosticLevel::Error,
                        message: Some(format!(
                            "incorrect event pointed to by tissue {}.",
                            index + 1
                        )),
                    });
                }

                None
            })
            .collect::<Vec<_>>()
    }
}
