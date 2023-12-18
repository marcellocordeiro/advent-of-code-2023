use crate::Game;

pub fn result(games: &[Game]) -> i32 {
    games
        .iter()
        .filter_map(|g| is_possible(g).then_some(g.id))
        .sum()
}

fn is_possible(game: &Game) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    game.plays
        .iter()
        .all(|play| play.red <= MAX_RED && play.green <= MAX_GREEN && play.blue <= MAX_BLUE)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_game, parse_input, INPUT, SAMPLE};

    #[test]
    fn test_each_sample_line() {
        let lines = SAMPLE.lines();
        let results = [true, true, false, false, true];

        assert_eq!(lines.clone().count(), results.len());

        for (line, expected_result) in lines.zip(results) {
            let game = parse_game(line);
            let actual_result = is_possible(&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_sample() {
        let games = parse_input(SAMPLE);
        let actual_result = result(&games);

        assert_eq!(actual_result, 8);
    }

    #[test]
    fn test_input() {
        let games = parse_input(INPUT);
        let actual_result = result(&games);

        assert_eq!(actual_result, 1853);
    }
}
