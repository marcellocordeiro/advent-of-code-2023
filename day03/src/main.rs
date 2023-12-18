use day03::{parse_input, part1, part2, INPUT};

fn main() {
    let (numbers, symbols) = parse_input(INPUT);

    let part1_result = part1::result(&numbers, &symbols);
    let part2_result = part2::result(&numbers, &symbols);

    println!("Part 1: {part1_result}");
    println!("Part 2: {part2_result}");
}
