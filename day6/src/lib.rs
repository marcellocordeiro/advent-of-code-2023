pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct Race {
    pub time_limit: usize,
    pub record_distance: usize,
}

pub fn win_count(race: &Race) -> usize {
    (1..(race.time_limit))
        .map(|t| (race.time_limit * t) - t * t)
        .filter(|d| *d > race.record_distance)
        .count()
}

#[allow(clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_sign_loss)]
pub fn win_count_optimized(race: &Race) -> usize {
    let (t1, t2) = {
        let t_limit = race.time_limit;
        let d_record = race.record_distance;

        let delta_sqrt = {
            let delta = t_limit.pow(2) - (4 * d_record);

            (delta as f64).sqrt()
        };

        (
            ((t_limit as f64) - delta_sqrt) / 2.0,
            ((t_limit as f64) + delta_sqrt) / 2.0,
        )
    };

    let min_t = if t1 == t1.ceil() {
        t1 as usize + 1
    } else {
        t1.ceil() as usize
    };

    let max_t = if t2 == t2.floor() {
        t2 as usize - 1
    } else {
        t2.floor() as usize
    };

    (max_t - min_t) + 1
}

pub mod part1;
pub mod part2;
