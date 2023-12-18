use day01::{parse_input, part1, part2, INPUT};

fn main() {
    let lines = parse_input(INPUT);

    let part1_result = part1::result(&lines);
    let part2_result = part2::result(&lines);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
