use itertools::Itertools;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("sample.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Object {
    Sphere, // Rounded rocks
    Rock,
    Empty,
}

impl Object {
    fn from_ch(ch: char) -> Self {
        match ch {
            'O' => Self::Sphere,
            '#' => Self::Rock,
            '.' => Self::Empty,

            _ => panic!("Invalid char"),
        }
    }

    fn to_ch(self) -> char {
        match self {
            Self::Sphere => 'O',
            Self::Rock => '#',
            Self::Empty => '.',
        }
    }
}

pub struct Platform {
    map: Vec<Vec<Object>>,
    rows: usize,
    columns: usize,
}

impl Platform {
    pub fn print(&self) {
        for row in &self.map {
            for column in row {
                print!("{}", column.to_ch());
            }

            println!();
        }
    }

    /*pub fn drop_north(&mut self) {
        let positions = (0..self.rows)
            .rev()
            .map(|i| (0..self.columns).map(|j| (i, j)).collect::<Vec<_>>());

        self.map.iter_mut().for_each(|row| {
            let rowww_it = row.split_mut(|obj| *obj == Object::Rock);

            rowww_it.for_each(|slice| {
                let slice_len = slice.len();
                let sphere_count = slice.iter().filter(|obj| **obj == Object::Sphere).count();

                let empty = [Object::Empty].repeat(slice_len - sphere_count);
                let spheres = [Object::Sphere].repeat(sphere_count);
                let joined = [empty, spheres].concat();

                for (old, new) in slice.iter_mut().zip(joined.iter()) {
                    *old = *new;
                }
            })
        });
    }*/

    pub fn total_load(&self) -> usize {
        let rows = self.rows;

        self.map
            .iter()
            .enumerate()
            .map(|(row_i, row)| {
                row.iter()
                    .map(|item| {
                        if *item == Object::Sphere {
                            rows - row_i
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    pub fn drop_north(&mut self) {
        let mut transposed = Self::transpose(&self.map);

        Self::drop_west_impl(&mut transposed);

        self.map = Self::transpose(&transposed);
    }

    pub fn drop_south(&mut self) {
        let mut transposed = Self::transpose(&self.map);

        Self::drop_east_impl(&mut transposed);

        self.map = Self::transpose(&transposed);
    }

    pub fn drop_west(&mut self) {
        Self::drop_west_impl(&mut self.map);
    }

    pub fn drop_east(&mut self) {
        Self::drop_east_impl(&mut self.map);
    }

    fn drop_west_impl(map: &mut Vec<Vec<Object>>) {
        map.iter_mut().for_each(|row| {
            row.split_mut(|obj| *obj == Object::Rock).for_each(|slice| {
                let slice_len = slice.len();
                let sphere_count = slice.iter().filter(|obj| **obj == Object::Sphere).count();

                let empty = [Object::Empty].repeat(slice_len - sphere_count);
                let spheres = [Object::Sphere].repeat(sphere_count);
                let joined = [spheres, empty].concat();

                for (old, new) in slice.iter_mut().zip(joined.iter()) {
                    *old = *new;
                }
            })
        });
    }

    fn drop_east_impl(map: &mut Vec<Vec<Object>>) {
        map.iter_mut().for_each(|row| {
            row.split_mut(|obj| *obj == Object::Rock).for_each(|slice| {
                let slice_len = slice.len();
                let sphere_count = slice.iter().filter(|obj| **obj == Object::Sphere).count();

                let empty = [Object::Empty].repeat(slice_len - sphere_count);
                let spheres = [Object::Sphere].repeat(sphere_count);
                let joined = [empty, spheres].concat();

                for (old, new) in slice.iter_mut().zip(joined.iter()) {
                    *old = *new;
                }
            })
        });
    }

    fn transpose(map: &Vec<Vec<Object>>) -> Vec<Vec<Object>> {
        let rows = map.len();
        let columns = map[0].len();

        (0..columns)
            .map(|col| (0..rows).map(|row| map[row][col]).collect())
            .collect()
    }
}

pub fn parse_input(input: &str) -> Platform {
    let map = input
        .lines()
        .map(|line| line.chars().map(Object::from_ch).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let rows = map.len();
    let columns = map[0].len();

    Platform { map, rows, columns }
}

pub mod part1;
pub mod part2;
