use itertools::Itertools;

use crate::{parse_input, Brick};

pub fn result(input: &str) -> usize {
    let mut bricks = parse_input(input);

    settle_bricks(&mut bricks);

    let ids = bricks
        .iter()
        .filter_map(|b| {
            let indexes = b.supporting(&bricks);

            (indexes.len() == 1).then_some(indexes)
        })
        .flatten()
        .unique()
        .collect_vec();

    // println!("IDs: {ids:?}");

    bricks.len() - ids.len()
}

fn settle_bricks(bricks: &mut [Brick]) {
    /*
    Brick #0: (1, 0, 1)~(1, 2, 1)
    Brick #1: (0, 0, 2)~(2, 0, 2)
    Brick #2: (0, 2, 2)~(2, 2, 2)
    Brick #3: (0, 0, 3)~(0, 2, 3)
    Brick #4: (2, 0, 3)~(2, 2, 3)
    Brick #5: (0, 1, 4)~(2, 1, 4)
    Brick #6: (1, 1, 5)~(1, 1, 6)
     */
    //for brick in bricks.iter() {
    //    println!("Brick #{}: {:?}~{:?}", brick.id, brick.from, brick.to);
    //}
    //println!();

    let max_z = bricks.iter().map(|b| b.from.2).max().unwrap();

    let mut modified = true;
    while modified {
        modified = false;

        for z in 1..=max_z {
            let in_line = bricks
                .iter()
                .enumerate()
                .filter_map(|(index, b)| (b.from.2 == z).then_some(index))
                .collect_vec();

            for brick_index in in_line {
                let supporting = bricks
                    .iter()
                    .filter(|b| {
                        b.id != bricks[brick_index].id && b.is_supporting(&bricks[brick_index])
                    })
                    .count();

                if supporting == 0 {
                    if bricks[brick_index].from.2 == 1 {
                        continue;
                    }

                    bricks[brick_index].from.2 -= 1;
                    bricks[brick_index].to.2 -= 1;
                    modified = true;
                }
            }
        }
    }

    //for brick in bricks.iter() {
    //    println!("Brick #{}: {:?}~{:?}", brick.id, brick.from, brick.to);
    //}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 5);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 485);
    }
}
