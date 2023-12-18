use crate::Game;

pub fn result(games: &[Game]) -> i32 {
    games.iter().map(each_result).sum()
}

fn each_result(game: &Game) -> i32 {
    game.plays
        .iter()
        .fold([0, 0, 0], |acc, play| {
            [
                acc[0].max(play.red),
                acc[1].max(play.green),
                acc[2].max(play.blue),
            ]
        })
        .iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{parse_game, parse_input, INPUT, SAMPLE};

    #[test]
    fn test_each_sample_line() {
        let lines = SAMPLE.lines();
        let results = [48, 12, 1560, 630, 36];

        assert_eq!(lines.clone().count(), results.len());

        for (line, expected_result) in lines.zip(results) {
            let game = parse_game(line);
            let actual_result = each_result(&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_sample() {
        let games = parse_input(SAMPLE);
        let actual_result = result(&games);

        assert_eq!(actual_result, 2286);
    }

    #[test]
    fn test_input() {
        let games = parse_input(INPUT);
        let actual_result = result(&games);

        assert_eq!(actual_result, 72706);
    }
}
