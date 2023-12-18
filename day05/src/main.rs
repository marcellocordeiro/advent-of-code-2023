use day05::{parse_input, part1, part2, INPUT};

fn main() {
    let almanac = parse_input(INPUT);

    let part1_result = part1::result(&almanac);
    let part2_result = part2::result(&almanac);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
