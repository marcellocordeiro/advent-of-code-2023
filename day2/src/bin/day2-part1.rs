use day2::{parse_games, Game, INPUT};

fn is_possible(game: &Game) -> bool {
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;

    game.plays
        .iter()
        .all(|play| play.red <= MAX_RED && play.green <= MAX_GREEN && play.blue <= MAX_BLUE)
}

fn result(games: &[Game]) -> i32 {
    games
        .iter()
        .filter(|game| is_possible(game))
        .fold(0, |acc, game| acc + game.id)
}

#[cfg(test)]
mod tests {
    use day2::parse_game;

    use super::*;

    #[test]
    fn test_each() {
        let lines = [
            (
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
                true,
            ),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                true,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                false,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                false,
            ),
            (
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
                true,
            ),
        ];

        for (line, expected_result) in lines {
            let game = parse_game(&line);
            let actual_result = is_possible(&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }
    }

    #[test]
    fn test_all() {
        let lines = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let expected_result = 8;

        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();
        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let games = parse_games(INPUT);
        let expected_result = 1853;

        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let games = parse_games(INPUT);
    let result = result(&games);

    println!("Result: {result}");
}
