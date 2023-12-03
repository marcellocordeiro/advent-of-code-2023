use std::ops::Range;

use regex::Regex;

pub const SAMPLE: &str = include_str!("sample.txt");
pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug)]
pub struct EngineNumber {
    pub number: i32,
    pub range: Range<usize>,
    pub line_index: usize,
}

#[derive(Debug)]
pub struct EngineSymbol {
    pub symbol: char,
    pub position: usize,
    pub line_index: usize,
}

pub fn convert_input(input: &str) -> Vec<&str> {
    input.trim().split('\n').collect()
}

pub fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_ascii_digit()
}

pub fn parse_engine(lines: &[&str]) -> (Vec<EngineNumber>, Vec<EngineSymbol>) {
    let re = Regex::new(r"\d+").unwrap();

    lines
        .iter()
        .enumerate()
        .fold((vec![], vec![]), |mut acc, (line_index, line)| {
            let mut numbers = re
                .find_iter(line)
                .map(|m| {
                    let number = m.as_str().parse::<i32>().unwrap();
                    let range = m.range();

                    EngineNumber {
                        number,
                        range,
                        line_index,
                    }
                })
                .collect();

            let mut symbols = line
                .chars()
                .enumerate()
                .filter_map(|(position, symbol)| {
                    if is_symbol(symbol) {
                        return Some(EngineSymbol {
                            symbol,
                            position,
                            line_index,
                        });
                    }

                    None
                })
                .collect();

            acc.0.append(&mut numbers);
            acc.1.append(&mut symbols);

            acc
        })
}
