use std::collections::HashMap;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE1: &str = include_str!("inputs/sample1.txt");
pub const SAMPLE2: &str = include_str!("inputs/sample2.txt");

#[derive(Debug, Clone)]
enum Module {
    Broadcaster {
        outputs: Vec<String>,
    },
    FlipFlop {
        state: bool,
        outputs: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, bool>,
        outputs: Vec<String>,
    },
    Untyped {
        state: bool,
    },
}

impl Module {
    fn send(&mut self, source: &str, pulse: bool) -> Option<(bool, Vec<String>)> {
        Some(match self {
            Self::Broadcaster { outputs } => (false, outputs.clone()),
            Self::FlipFlop { state, outputs } => {
                if pulse {
                    return None;
                }

                *state = !*state;

                (*state, outputs.clone())
            }
            Self::Conjunction { inputs, outputs } => {
                *inputs.get_mut(source).unwrap() = pulse;

                let pulse = !inputs.values().all(|v| *v);

                (pulse, outputs.clone())
            }
            Self::Untyped { state } => {
                *state = pulse;
                return None;
            }
        })
    }
}

fn parse_input(input: &str) -> HashMap<String, Module> {
    let original_modules = input
        .lines()
        .map(|line| {
            let (from, to) = line.split_once(" -> ").unwrap();

            let outputs = to.split(", ").map(str::to_owned).collect::<Vec<_>>();

            if from == "broadcaster" {
                ("broadcaster".to_owned(), Module::Broadcaster { outputs })
            } else if let Some(from) = from.strip_prefix('%') {
                (
                    from.to_owned(),
                    Module::FlipFlop {
                        state: false,
                        outputs,
                    },
                )
            } else if let Some(from) = from.strip_prefix('&') {
                (
                    from.to_owned(),
                    Module::Conjunction {
                        inputs: HashMap::new(),
                        outputs,
                    },
                )
            } else {
                panic!("Invalid");
            }
        })
        .collect::<HashMap<_, _>>();

    let mut adjusted_modules = original_modules.clone();

    adjusted_modules
        .iter_mut()
        .filter_map(|(key, value)| match value {
            Module::Conjunction { inputs, .. } => Some((key, inputs)),
            _ => None,
        })
        .for_each(|(key, inputs)| {
            for (o_key, o_value) in &original_modules {
                if *key == *o_key {
                    continue;
                }

                match o_value {
                    Module::Broadcaster { outputs }
                    | Module::FlipFlop { outputs, .. }
                    | Module::Conjunction { outputs, .. } => {
                        if outputs.contains(key) {
                            inputs.insert(o_key.clone(), false);
                        }
                    }

                    Module::Untyped { .. } => {}
                }
            }
        });

    for entry in original_modules.values() {
        match entry {
            Module::Broadcaster { outputs }
            | Module::FlipFlop { outputs, .. }
            | Module::Conjunction { outputs, .. } => {
                for output in outputs {
                    if !adjusted_modules.contains_key(output) {
                        adjusted_modules.insert(output.clone(), Module::Untyped { state: false });
                    }
                }
            }

            Module::Untyped { .. } => continue,
        }
    }

    adjusted_modules
}

pub mod part1;
pub mod part2;
