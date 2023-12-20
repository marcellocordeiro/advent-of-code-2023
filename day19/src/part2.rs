use crate::{parse_input, Action, Compare, Rule, Workflow};

pub fn result(input: &str) -> usize {
    let (workflows, _) = parse_input(input);

    let selected_workflow = workflows.iter().find(|w| w.name == "in").unwrap();
    let ranges = RatingRanges::new();

    get_combinations(ranges, selected_workflow, &workflows)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RatingRanges {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl RatingRanges {
    fn new() -> Self {
        let default = (1, 4000);

        Self {
            x: default,
            m: default,
            a: default,
            s: default,
        }
    }

    fn get_from_str(&self, rating: &str) -> (usize, usize) {
        match rating {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,

            _ => panic!("Invalid rating"),
        }
    }

    fn set_from_str(&mut self, rating: &str, range: (usize, usize)) {
        match rating {
            "x" => self.x = range,
            "m" => self.m = range,
            "a" => self.a = range,
            "s" => self.s = range,

            _ => panic!("Invalid rating"),
        }
    }

    fn get_combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn adjust_range(&mut self, rating: &str, compare: Compare) {
        let current_range = self.get_from_str(rating);

        let new_range = match compare {
            Compare::Less(operand) => (current_range.0, current_range.1.min(operand - 1)),
            Compare::Greater(operand) => (current_range.0.max(operand + 1), current_range.1),
        };

        self.set_from_str(rating, new_range);
    }

    fn is_valid(&self) -> bool {
        (self.x.1 >= self.x.0)
            && (self.m.1 >= self.m.0)
            && (self.a.1 >= self.a.0)
            && (self.s.1 >= self.s.0)
    }
}

fn get_combinations(
    ranges: RatingRanges,
    selected_workflow: &Workflow,
    workflows: &[Workflow],
) -> usize {
    let mut current_ranges = ranges;
    let mut current_result = 0;

    for rule in &selected_workflow.rules {
        match rule {
            Rule::Test {
                rating,
                compare,
                action: Action::MoveTo(name),
            } => {
                // Try the next workflow
                let next_ranges = {
                    let mut r = current_ranges.clone();
                    r.adjust_range(rating, *compare);

                    r
                };

                let next_workflow = workflows.iter().find(|w| w.name == *name).unwrap();

                if next_ranges.is_valid() {
                    current_result += get_combinations(next_ranges, next_workflow, workflows);
                }

                // Flip and keep going...

                // Get range difference
                current_ranges.adjust_range(rating, compare.flip());

                assert!(current_ranges.is_valid(), "Adjusted range is not valid");

                // Keep going...
            }

            Rule::Test {
                rating,
                compare,
                action: Action::Accept,
            } => {
                // Maybe accepted
                let next_ranges = {
                    let mut r = current_ranges.clone();
                    r.adjust_range(rating, *compare);

                    r
                };

                if next_ranges.is_valid() {
                    // Accepted!
                    current_result += next_ranges.get_combinations();
                } else {
                    panic!("Invalid range");
                }

                // Flip and keep going...

                // Get range difference
                current_ranges.adjust_range(rating, compare.flip());

                assert!(current_ranges.is_valid(), "Adjusted range is not valid");

                // Keep going...
            }

            Rule::Test {
                rating,
                compare,
                action: Action::Reject,
            } => {
                current_ranges.adjust_range(rating, compare.flip());

                assert!(current_ranges.is_valid(), "Adjusted range is not valid");

                // Keep going...
            }

            Rule::Action(Action::MoveTo(name)) => {
                let next_workflow = workflows.iter().find(|w| w.name == *name).unwrap();

                return current_result + get_combinations(current_ranges, next_workflow, workflows);
            }

            Rule::Action(Action::Accept) => {
                // Accepted!
                current_result += current_ranges.get_combinations();

                return current_result;
            }

            Rule::Action(Action::Reject) => {
                // Rejected, return what we have so far
                return current_result;
            }
        };
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 125355665599537);
    }
}
