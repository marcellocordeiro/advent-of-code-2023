use std::collections::{HashMap, VecDeque};

use num::Integer;

use crate::{parse_input, Module};

// We need to make assumptions based on our input. So none of this code looks good :(
// I will try to refactor it at some point.
/*
   "tg": Conjunction {
       inputs: {
           "vq": false,
           "tf": false,
           "db": false,
           "ln": false,
       },
       outputs: [
           "rx",
       ],
   },
*/

pub fn result(input: &str) -> usize {
    let mut modules = parse_input(input);
    let mut queue = VecDeque::new();

    let mut button_presses = 0;

    let tg_inputs = modules
        .iter()
        .find(|(key, _module)| *key == "tg")
        .map(|(_, module)| match module {
            Module::Conjunction { inputs, .. } => inputs,
            _ => panic!("Invalid module"),
        })
        .unwrap();

    let mut tg_count_map = tg_inputs
        .keys()
        .map(|input| (input.clone(), None))
        .collect::<HashMap<String, Option<usize>>>();

    let broadcast_outputs = modules
        .values()
        .find_map(|m| match m {
            Module::Broadcaster { outputs } => Some(outputs),
            _ => None,
        })
        .unwrap()
        .clone();

    loop {
        queue.push_back(("broadcaster".to_owned(), false, broadcast_outputs.clone()));
        button_presses += 1;

        while let Some((source, pulse, outputs)) = queue.pop_front() {
            for output in outputs {
                if tg_count_map.contains_key(&source) && output == "tg" && pulse {
                    if let Some(value) = tg_count_map.get_mut(&source) {
                        match value {
                            None => *value = Some(button_presses),
                            Some(_) => {}
                        }
                    }

                    if tg_count_map.values().all(Option::is_some) {
                        return tg_count_map
                            .values()
                            .map(|value| value.unwrap())
                            .reduce(|acc, value| acc.lcm(&value))
                            .unwrap();
                    }
                }

                if let Some((next_pulse, next_outputs)) =
                    modules.get_mut(&output).unwrap().send(&source, pulse)
                {
                    queue.push_back((output, next_pulse, next_outputs));
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::INPUT;

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 252667369442479);
    }
}
