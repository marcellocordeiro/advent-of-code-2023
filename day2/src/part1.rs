use crate::Game;

fn is_possible(game: &Game) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    game.plays
        .iter()
        .all(|play| play.red <= MAX_RED && play.green <= MAX_GREEN && play.blue <= MAX_BLUE)
}

pub fn result(games: &[Game]) -> i32 {
    games
        .iter()
        .filter(|game| is_possible(game))
        .fold(0, |acc, game| acc + game.id)
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use common::split_by_line;

    use crate::{parse_game, parse_games, INPUT, SAMPLE};

    use super::*;

    #[test]
    fn test_each() {
        let lines = split_by_line(SAMPLE);
        let results = [true, true, false, false, true];

        assert_eq!(lines.len(), results.len());

        let zipped = zip(lines, results);

        for (line, expected_result) in zipped {
            let game = parse_game(&line);
            let actual_result = is_possible(&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_all() {
        let lines = split_by_line(SAMPLE);

        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();
        let actual_result = result(&games);

        assert_eq!(actual_result, 8);
    }

    #[test]
    fn test_input() {
        let games = parse_games(INPUT);
        let actual_result = result(&games);

        assert_eq!(actual_result, 1853);
    }
}
