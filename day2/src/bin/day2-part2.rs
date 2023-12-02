use day2::{parse_games, Game, INPUT};

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

fn result(games: &[Game]) -> i32 {
    games.iter().map(each_result).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use day2::parse_game;

    #[test]
    fn test_each() {
        let lines = [
            ("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48),
            (
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
                12,
            ),
            (
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                1560,
            ),
            (
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                630,
            ),
            ("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36),
        ];

        for (line, expected_result) in lines {
            let game = parse_game(&line);
            let actual_result = each_result(&&game);

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
        let expected_result = 2286;

        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();
        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn test_input() {
        let expected_result = 72706;

        let lines = INPUT.trim().split('\n').collect::<Vec<&str>>();
        let games = lines.into_iter().map(parse_game).collect::<Vec<Game>>();

        let actual_result = result(&games);

        assert_eq!(actual_result, expected_result);
    }
}

fn main() {
    let games = parse_games(INPUT);

    let result = result(&games);

    println!("Result: {result}");
}
