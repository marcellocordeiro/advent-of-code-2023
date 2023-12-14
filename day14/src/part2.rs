use crate::{parse_input, Object, Platform};

pub fn result(input: &str) -> usize {
    let mut platform = parse_input(input);

    let mut history = Vec::<Vec<Vec<Object>>>::new();
    let mut cycle_length = None;

    for cycles in 0..1_000_000_000 {
        history.push(platform.map.clone());

        platform.drop_north();
        platform.drop_west();
        platform.drop_south();
        platform.drop_east();

        if let Some(history_cycle) = history.iter().position(|entry| *entry == platform.map) {
            let cycle_len = cycles - history_cycle + 1;
            let index = (1_000_000_000 - history_cycle + 1) % cycle_len + history_cycle - 1;
            cycle_length = Some(index);
            break;
        }
    }

    if let Some(index) = cycle_length {
        Platform::total_load(&history[index])
    } else {
        Platform::total_load(&platform.map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 64);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 106689);
    }
}
