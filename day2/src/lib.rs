pub struct Play {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub struct Game {
    pub id: i32,
    pub plays: Vec<Play>,
}

pub fn parse_games(input: &str) -> Vec<Game> {
    let lines = input.trim().split('\n').collect::<Vec<&str>>();

    lines.into_iter().map(parse_game).collect::<Vec<Game>>()
}

pub fn parse_game(line: &str) -> Game {
    let (id, raw_plays) = {
        let split = line.split(':').map(str::trim).collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);

        let id = split[0].replace("Game ", "").parse::<i32>().unwrap();
        let raw_plays = split[1].split(';').map(str::trim).collect::<Vec<&str>>();

        (id, raw_plays)
    };

    let plays = raw_plays
        .into_iter()
        .map(|raw_play| {
            raw_play.split(", ").fold(
                Play {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |mut acc, raw_cubes_with_count| {
                    let split = raw_cubes_with_count.split(' ').collect::<Vec<&str>>();
                    assert_eq!(split.len(), 2);

                    let count = split[0].parse::<i32>().unwrap();
                    let colour = split[1];

                    match colour {
                        "red" => acc.red += count,
                        "green" => acc.green += count,
                        "blue" => acc.blue += count,

                        _ => panic!("Invalid colour: {colour}"),
                    }

                    acc
                },
            )
        })
        .collect::<Vec<Play>>();

    Game { id, plays }
}


