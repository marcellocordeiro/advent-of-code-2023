use crate::{parse_input, Action, PartRatings, Rule, Workflow};

pub fn result(input: &str) -> usize {
    let (workflows, all_part_ratings) = parse_input(input);

    all_part_ratings
        .into_iter()
        .filter(|part_ratings| {
            let mut current_workflow = workflows.iter().find(|w| w.name == "in").unwrap();

            loop {
                let next_action = test_part_with_workflow(part_ratings, current_workflow);

                match next_action {
                    Action::MoveTo(workflow_name) => {
                        current_workflow =
                            workflows.iter().find(|w| w.name == workflow_name).unwrap();
                    }
                    Action::Reject => return false,
                    Action::Accept => return true,
                }
            }
        })
        .map(|part_ratings| part_ratings.x + part_ratings.m + part_ratings.a + part_ratings.s)
        .sum()
}

fn test_part_with_workflow(part_ratings: &PartRatings, workflow: &Workflow) -> Action {
    for rule in &workflow.rules {
        match rule {
            Rule::Test {
                part,
                compare,
                action,
            } => {
                if compare.test_with(part_ratings.get_from_str(part)) {
                    return action.clone();
                }
            }

            Rule::Action(action) => return action.clone(),
        }
    }

    panic!("Unable to complete the workflow");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 19114);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 353046);
    }
}
