use day04::{parse_input, part1, part2, INPUT};

fn main() {
    let cards = parse_input(INPUT);

    let part1_result = part1::result(&cards);
    let part2_result = part2::result(&cards);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
