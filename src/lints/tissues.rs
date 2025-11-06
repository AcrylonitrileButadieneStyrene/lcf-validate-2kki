use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

pub struct TissueLint;

impl super::Lint for TissueLint {
    fn name(&self) -> &'static str {
        "Tissue event validity"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let main_sig = encoding_rs::SHIFT_JIS.encode("ティッシュ++++").0.to_vec();
        let helper_sig = encoding_rs::SHIFT_JIS.encode("ティッシュ").0.to_vec();

        let Some(tissue) = map.events.iter().find(|event| event.name == main_sig) else {
            return Diagnostic::Warning("Does not have tissue events".to_string());
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
            return Diagnostic::Error(format!(
                "Expected 5 tissue helper events but found {}",
                helpers.len()
            ));
        }

        let failures = helpers
            .iter()
            .enumerate()
            .filter_map(|(index, id)| {
                let Some(event) = map.events.iter().find(|event| event.id == *id) else {
                    return Some(format!(
                        "\n    Helper event {} points to non-existent EV{id:04}",
                        index + 1
                    ));
                };

                if !event.name.starts_with(&helper_sig) {
                    return Some(format!(
                        "\n    Helper event {} points to incorrect event EV{id:04} ({})",
                        index + 1,
                        encoding_rs::SHIFT_JIS.decode(&event.name).0,
                    ));
                }

                None
            })
            .collect::<Vec<_>>();

        if failures.is_empty() {
            Diagnostic::Normal
        } else {
            Diagnostic::Error(failures.into_iter().collect())
        }
    }
}
