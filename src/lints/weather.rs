use lcf::raw::lmu::event::instruction::Instruction;

pub struct WeatherLint;

enum State {
    Normal,
    ExpectingWeather,
    ExpectingVariable,
    Finished,
}

impl super::Lint for WeatherLint {
    fn name(&self) -> &'static str {
        "Parity between weather and V0042"
    }

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Vec<super::Diagnostic> {
        let mut state = State::Normal;
        let mut diagnostics = Vec::new();

        for event in &map.events {
            for (page_index, page) in event.pages.iter().enumerate() {
                for command in &page.commands {
                    match command.instruction {
                        Instruction::WeatherEffects { .. } => {
                            state = match state {
                                State::Normal => State::ExpectingVariable,
                                State::ExpectingWeather => State::Finished,
                                x => x,
                            }
                        }
                        Instruction::ControlVariables {
                            mode, start, end, ..
                        } => {
                            if match mode {
                                0 => start == 42,
                                1 => start <= 42 && 42 <= end,
                                _ => false,
                            } {
                                state = match state {
                                    State::Normal => State::ExpectingWeather,
                                    State::ExpectingVariable => State::Finished,
                                    x => x,
                                }
                            }
                        }
                        _ => (),
                    }
                }

                // todo: find index of last command
                diagnostics.push(super::Diagnostic {
                    event: Some(super::DiagnosticEvent::from(event).with_page(
                        super::DiagnosticPage::new_from_index(page_index),
                    )),
                    level: super::DiagnosticLevel::Error,
                    message: Some(match state {
                        State::ExpectingVariable => {
                            state = State::Normal;
                            "V0042 is not changed after changing the weather.".to_string()
                        }
                        State::ExpectingWeather => {
                            state = State::Normal;
                            "The weather is not changed after changing V0042.".to_string()
                        }
                        _ => continue,
                    }),
                });
            }
        }

        diagnostics
    }
}
