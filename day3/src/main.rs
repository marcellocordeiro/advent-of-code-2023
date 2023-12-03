use common::split_by_line;
use day3::{parse_engine, part1, part2, INPUT};

fn main() {
    let lines = split_by_line(INPUT);
    let (numbers, symbols) = parse_engine(&lines);

    let part1_result = part1::result(&numbers, &symbols);
    let part2_result = part2::result(&numbers, &symbols);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
