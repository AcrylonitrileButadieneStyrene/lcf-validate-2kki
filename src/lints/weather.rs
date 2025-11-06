use lcf::raw::lmu::event::instruction::Instruction;

use crate::Diagnostic;

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

    fn test(&self, map: &lcf::lmu::LcfMapUnit) -> Diagnostic {
        let mut state = State::Normal;
        let mut failures = Vec::new();

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

                match state {
                    State::ExpectingVariable => {
                        failures.push((true, event, page_index + 1));
                        state = State::Normal;
                    }
                    State::ExpectingWeather => {
                        failures.push((false, event, page_index + 1));
                        state = State::Normal;
                    }
                    _ => (),
                }
            }
        }

        if failures.is_empty() {
            Diagnostic::Normal
        } else {
            Diagnostic::Error(failures
                .into_iter()
                .map(|(is_var, event, page)| {
                    let msg = if is_var {
                        "the weather after changing V0042"
                    } else {
                        "V0042 after changing the weather"
                    };

                    format!(
                        "\n    EV{:04} (X{:03}, Y{:03}) on page {page:02}: Expected a change to {msg}, but none was found.",
                        event.id, event.x, event.y
                    )
                })
                .collect())
        }
    }
}
