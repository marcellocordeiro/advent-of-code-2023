use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

pub struct EngineNumber {
    pub number: i32,
    pub column_range: Range<usize>,
    pub row: usize,
}

pub struct EngineSymbol {
    pub symbol: char,
    pub column: usize,
    pub row: usize,
}

pub fn parse_input(input: &str) -> (Vec<EngineNumber>, Vec<EngineSymbol>) {
    let re = Regex::new(r"\d+").unwrap();

    input
        .lines()
        .enumerate()
        .fold((vec![], vec![]), |mut acc, (row, line)| {
            let mut numbers = re
                .find_iter(line)
                .map(|m| {
                    let number = m.as_str().parse().unwrap();
                    let column_range = m.range();

                    EngineNumber {
                        number,
                        column_range,
                        row,
                    }
                })
                .collect();

            let mut symbols = line
                .chars()
                .enumerate()
                .filter_map(|(column, symbol)| {
                    is_symbol(symbol).then_some(EngineSymbol {
                        symbol,
                        column,
                        row,
                    })
                })
                .collect();

            acc.0.append(&mut numbers);
            acc.1.append(&mut symbols);

            acc
        })
}

pub fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_ascii_digit()
}

pub fn get_surrounding_coordinates(
    (row, col): (usize, usize),
    //(max_row, max_col): (usize, usize),
) -> Vec<(usize, usize)> {
    let row_start = row.saturating_sub(1);
    let row_end = row.saturating_add(1); //.min(max_row);

    let col_start = col.saturating_sub(1);
    let col_end = col.saturating_add(1); //.min(max_col);

    let row_range = row_start..=row_end;
    let col_range = col_start..=col_end;

    row_range.cartesian_product(col_range).unique().collect()
}

pub mod part1;
pub mod part2;
