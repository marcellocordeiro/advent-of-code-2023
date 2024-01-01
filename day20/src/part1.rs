use std::collections::VecDeque;

use crate::parse_input;

pub fn result(input: &str) -> usize {
    let mut modules = parse_input(input);
    let mut queue = VecDeque::new();

    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1000 {
        let aa = modules
            .iter_mut()
            .find(|(key, _)| *key == "broadcaster")
            .map(|(_, module)| module.send("button", false))
            .unwrap()
            .unwrap();

        low_pulses += 1;

        queue.push_back(("broadcaster".to_owned(), aa.0, aa.1));

        // println!("button -low-> broadcaster");

        while let Some((source, pulse, outputs)) = queue.pop_front() {
            for output in outputs {
                /*println!(
                    "{source} -{}-> {output}",
                    if pulse { "high" } else { "low" }
                );*/

                if pulse {
                    high_pulses += 1;
                } else {
                    low_pulses += 1;
                }

                if let Some((next_pulse, next_outputs)) =
                    modules.get_mut(&output).unwrap().send(&source, pulse)
                {
                    queue.push_back((output, next_pulse, next_outputs));
                }
            }
        }
    }

    low_pulses * high_pulses
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE1, SAMPLE2};

    #[test]
    fn test_sample1() {
        let input = SAMPLE1;

        let result = result(input);

        assert_eq!(result, 32000000);
    }

    #[test]
    fn test_sample2() {
        let input = SAMPLE2;

        let result = result(input);

        assert_eq!(result, 11687500);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 819397964);
    }
}
