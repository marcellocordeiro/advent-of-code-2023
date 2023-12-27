use crate::{get_proper_min_location, Almanac};

pub fn result(almanac: &Almanac) -> usize {
    let seed_ranges = almanac
        .seeds
        .chunks_exact(2)
        .map(|chunk| {
            let [src_start, length] = chunk.try_into().unwrap();

            src_start..(src_start + length)
        })
        .collect::<Vec<_>>();

    seed_ranges
        .iter()
        .flat_map(|range| {
            range
                .clone()
                .map(|seed| get_proper_min_location(seed, &almanac.maps))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_input, INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let almanac = parse_input(SAMPLE);

        let result = result(&almanac);

        assert_eq!(result, 46);
    }

    #[test]
    #[ignore = "takes very long to compute"]
    fn test_input() {
        let almanac = parse_input(INPUT);

        let result = result(&almanac);

        assert_eq!(result, 69323688);
    }
}
