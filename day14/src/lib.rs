pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

    pub fn total_load(map: &Vec<Vec<Object>>) -> usize {
        let rows = map.len();

        map.iter()
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
        let mut modified = true;

        while modified {
            modified = false;

            for j in 0..self.columns {
                for i in 1..self.rows {
                    if self.map[i - 1][j] == Object::Empty && self.map[i][j] == Object::Sphere {
                        self.map[i - 1][j] = Object::Sphere;
                        self.map[i][j] = Object::Empty;
                        modified = true;
                    }
                }
            }
        }
    }

    pub fn drop_south(&mut self) {
        let mut modified = true;

        while modified {
            modified = false;

            for j in 0..self.columns {
                for i in 1..self.rows {
                    if self.map[i][j] == Object::Empty && self.map[i - 1][j] == Object::Sphere {
                        self.map[i][j] = Object::Sphere;
                        self.map[i - 1][j] = Object::Empty;
                        modified = true;
                    }
                }
            }
        }
    }

    pub fn drop_west(&mut self) {
        let mut modified = true;

        while modified {
            modified = false;

            for i in 0..self.rows {
                for j in 1..self.columns {
                    if self.map[i][j - 1] == Object::Empty && self.map[i][j] == Object::Sphere {
                        self.map[i][j - 1] = Object::Sphere;
                        self.map[i][j] = Object::Empty;
                        modified = true;
                    }
                }
            }
        }
    }

    pub fn drop_east(&mut self) {
        let mut modified = true;

        while modified {
            modified = false;

            for i in 0..self.rows {
                for j in 1..self.columns {
                    if self.map[i][j] == Object::Empty && self.map[i][j - 1] == Object::Sphere {
                        self.map[i][j] = Object::Sphere;
                        self.map[i][j - 1] = Object::Empty;
                        modified = true;
                    }
                }
            }
        }
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
