use crate::{win_count_optimized, Race};

pub fn result(input: &str) -> usize {
    let races = parse_input(input);
    races.iter().map(win_count_optimized).product()
}

fn parse_input(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = times
        .replace("Time:", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    let distances = distances
        .replace("Distance:", "")
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(times.len(), distances.len());

    times
        .into_iter()
        .zip(distances)
        .map(|(time_limit, record_distance)| Race {
            time_limit,
            record_distance,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{win_count, INPUT, SAMPLE};

    #[test]
    fn test_each_sample_line() {
        let input = SAMPLE;
        let races = parse_input(input);
        let results = [4, 8, 9];

        for (race, expected_result) in races.iter().zip(results) {
            let result = win_count_optimized(race);

            assert_eq!(
                result, expected_result,
                "for t = {}, d = {}",
                race.time_limit, race.record_distance
            );
        }
    }

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 288);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 741000);
    }

    #[test]
    fn test_optimized_solution() {
        let inputs = [SAMPLE, INPUT];

        for input in inputs {
            let races = parse_input(input);

            for race in races {
                assert_eq!(win_count(&race), win_count_optimized(&race));
            }
        }
    }
}
