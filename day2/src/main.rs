use day2::{parse_games, part1, part2, INPUT};

fn main() {
    let games = parse_games(INPUT);

    let part1_result = part1::result(&games);
    let part2_result = part2::result(&games);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
