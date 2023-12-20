pub const INPUT: &str = include_str!("inputs/input.txt");
pub const SAMPLE: &str = include_str!("inputs/sample.txt");

#[derive(Debug, Clone, Copy)]
enum Compare {
    Less(usize),
    Greater(usize),
}

impl Compare {
    fn flip(self) -> Self {
        match self {
            Self::Less(operand) => Self::Greater(operand - 1),
            Self::Greater(operand) => Self::Less(operand + 1),
        }
    }

    fn test_with(self, value: usize) -> bool {
        match self {
            Self::Less(operand) => value < operand,
            Self::Greater(operand) => value > operand,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Action {
    MoveTo(String),
    Reject,
    Accept,
}

impl Action {
    fn from_str(raw: &str) -> Self {
        match raw {
            "R" => Self::Reject,
            "A" => Self::Accept,
            part => Self::MoveTo(part.to_owned()),
        }
    }
}

#[derive(Debug)]
enum Rule {
    Test {
        rating: String,
        compare: Compare,
        action: Action,
    },
    Action(Action),
}

impl Rule {
    fn from_str(raw: &str) -> Self {
        let parts = raw.split(':').collect::<Vec<_>>();

        if parts.len() == 1 {
            Self::Action(Action::from_str(parts[0]))
        } else if parts.len() == 2 {
            let test_part = parts[0];
            let action = Action::from_str(parts[1]);

            if test_part.contains('<') {
                let (part, value) = test_part.split_once('<').unwrap();

                let part = part.to_owned();
                let value = value.parse().unwrap();

                Self::Test {
                    rating: part,
                    compare: Compare::Less(value),
                    action,
                }
            } else if test_part.contains('>') {
                let (part, value) = test_part.split_once('>').unwrap();

                let part = part.to_owned();
                let value = value.parse().unwrap();

                Self::Test {
                    rating: part,
                    compare: Compare::Greater(value),
                    action,
                }
            } else {
                panic!("Invalid format");
            }
        } else {
            panic!("Invalid rule");
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn from_str(raw: &str) -> Self {
        let (name, rules) = raw.split_once('{').unwrap();

        let name = name.to_owned();
        let rules = rules[..(rules.len() - 1)]
            .split(',')
            .map(Rule::from_str)
            .collect::<Vec<_>>();

        Self { name, rules }
    }
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    #[must_use]
    fn from_str(raw: &str) -> Self {
        let raw = &raw[1..(raw.len() - 1)];

        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;

        for item in raw.split(',') {
            let (rating, value) = item.split_once('=').unwrap();

            match rating {
                "x" => x = Some(value.parse().unwrap()),
                "m" => m = Some(value.parse().unwrap()),
                "a" => a = Some(value.parse().unwrap()),
                "s" => s = Some(value.parse().unwrap()),

                _ => panic!("Invalid variable"),
            }
        }

        Self {
            x: x.unwrap(),
            m: m.unwrap(),
            a: a.unwrap(),
            s: s.unwrap(),
        }
    }

    fn get_from_str(&self, rating: &str) -> usize {
        match rating {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,

            _ => panic!("Invalid rating"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (workflows, ratings) = input.trim().split_once("\n\n").unwrap();

    let workflows = workflows.lines().map(Workflow::from_str).collect();
    let ratings = ratings.lines().map(Part::from_str).collect();

    (workflows, ratings)
}

pub mod part1;
pub mod part2;
