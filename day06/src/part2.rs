use crate::{win_count_optimized, Race};

pub fn result(input: &str) -> usize {
    let race = parse_input(input);

    win_count_optimized(&race)
}

fn parse_input(input: &str) -> Race {
    let (times, distances) = input.trim().split_once('\n').unwrap();

    let time_limit = times
        .strip_prefix("Time:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    let record_distance = distances
        .strip_prefix("Distance:")
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    Race {
        time_limit,
        record_distance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{win_count, INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 71503);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 38220708);
    }

    #[test]
    fn test_optimized_solution() {
        let inputs = [SAMPLE, INPUT];

        for input in inputs {
            let race = parse_input(input);

            assert_eq!(win_count(&race), win_count_optimized(&race));
        }
    }
}
