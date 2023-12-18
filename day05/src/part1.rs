use crate::{get_proper_min_location, Almanac};

pub fn result(almanac: &Almanac) -> usize {
    almanac
        .seeds
        .iter()
        .map(|seed| get_proper_min_location(*seed, &almanac.maps))
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

        let min = result(&almanac);

        assert_eq!(min, 35);
    }

    #[test]
    fn test_input() {
        let almanac = parse_input(INPUT);

        let min = result(&almanac);

        assert_eq!(min, 111_627_841);
    }
}
