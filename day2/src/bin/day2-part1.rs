use day2::{parse_games, Game};

fn is_possible(game: &&Game) -> bool {
    // 12 red cubes, 13 green cubes, and 14 blue cubes
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
        .filter(is_possible)
        .fold(0, |acc, game| acc + game.id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use day2::parse_game;

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
        let expected_result = 8;

        for (line, expected_result) in lines {
            let game = parse_game(&line);
            let actual_result = is_possible(&&game);

            assert_eq!(actual_result, expected_result, "for {line}");
        }

        let games = lines
            .into_iter()
            .map(|(g, _)| parse_game(g))
            .collect::<Vec<Game>>();
        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
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
        let input = include_str!("../input.txt");
        let expected_result = 1853;

        let lines: Vec<&str> = input.trim().split('\n').collect();
        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();

        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let games = parse_games(input);

    let result = result(&games);

    println!("Result: {result}");
}
