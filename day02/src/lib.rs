pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Play {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub struct Game {
    pub id: i32,
    pub plays: Vec<Play>,
}

pub fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_game).collect()
}

pub fn parse_game(line: &str) -> Game {
    let (id, raw_plays) = {
        let (id_part, plays_part) = line.split_once(": ").unwrap();

        let id = id_part.replace("Game ", "").parse().unwrap();
        let raw_plays = plays_part.split("; ").collect::<Vec<_>>();

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
                    let (count, color) = raw_cubes_with_count
                        .split_once(' ')
                        .map(|(a, b)| (a.parse::<i32>().unwrap(), b))
                        .unwrap();

                    match color {
                        "red" => acc.red += count,
                        "green" => acc.green += count,
                        "blue" => acc.blue += count,

                        _ => panic!("Invalid color: {color}"),
                    }

                    acc
                },
            )
        })
        .collect();

    Game { id, plays }
}

pub mod part1;
pub mod part2;
