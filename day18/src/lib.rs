pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

struct Entry {
    direction: Direction,
    steps: isize,
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_ch(ch: char) -> Self {
        use Direction::{East, North, South, West};
        match ch {
            'U' => North,
            'D' => South,
            'R' => East,
            'L' => West,

            _ => unreachable!(),
        }
    }

    fn from_hex_ch(ch: char) -> Self {
        use Direction::{East, North, South, West};
        match ch {
            '0' => East,
            '1' => South,
            '2' => West,
            '3' => North,

            _ => unreachable!(),
        }
    }

    #[must_use]
    fn offset(self) -> (isize, isize) {
        use Direction::{East, North, South, West};
        match self {
            North => (-1, 0),
            South => (1, 0),
            East => (0, 1),
            West => (0, -1),
        }
    }
}

fn calculate_area(entries: Vec<Entry>) -> isize {
    let mut vertices = Vec::new();
    let mut external_area = 0;

    let mut cur_i = 0_isize;
    let mut cur_j = 0_isize;

    vertices.push((cur_i, cur_j));

    for Entry { direction, steps } in entries {
        let (i_offset, j_offset) = {
            let (i, j) = direction.offset();
            (i * steps, j * steps)
        };

        cur_i += i_offset;
        cur_j += j_offset;

        vertices.push((cur_i, cur_j));
        external_area += steps;
    }

    // Area inside (Shoelace formula)
    // https://en.wikipedia.org/wiki/Shoelace_formula
    let mut area = 0_isize;
    for w in vertices.windows(2) {
        area += w[0].0 * w[1].1;
        area -= w[0].1 * w[1].0;
    }

    let area = isize::abs(area) / 2;

    // Tiles inside (Pick's theorem)
    // https://en.wikipedia.org/wiki/Pick's_theorem
    let points_inside = area - (external_area / 2) + 1;

    points_inside + external_area
}

pub mod part1;
pub mod part2;
