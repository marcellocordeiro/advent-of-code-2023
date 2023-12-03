use crate::Game;

fn each_result(game: &Game) -> i32 {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for play in &game.plays {
        red = red.max(play.red);
        green = green.max(play.green);
        blue = blue.max(play.blue);
    }

    red * green * blue
}

pub fn result(games: &[Game]) -> i32 {
    games.iter().map(each_result).sum()
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
        let results = [48, 12, 1560, 630, 36];

        assert_eq!(lines.len(), results.len());

        let zipped = zip(lines, results);

        for (line, expected_result) in zipped {
            let game = parse_game(&line);
            let actual_result = each_result(&&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_all() {
        let lines = split_by_line(SAMPLE);

        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();
        let actual_result = result(&games);

        assert_eq!(actual_result, 2286);
    }

    #[test]
    fn test_input() {
        let games = parse_games(INPUT);
        let actual_result = result(&games);

        assert_eq!(actual_result, 72706);
    }
}
