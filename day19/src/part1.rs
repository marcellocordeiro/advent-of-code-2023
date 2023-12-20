use crate::{parse_input, Action, Part, Rule, Workflow};

pub fn result(input: &str) -> usize {
    let (workflows, parts) = parse_input(input);

    parts
        .into_iter()
        .filter(|part| {
            let mut current_workflow = workflows.iter().find(|w| w.name == "in").unwrap();

            loop {
                let next_action = test_part_with_workflow(part, current_workflow);

                match next_action {
                    Action::MoveTo(name) => {
                        current_workflow = workflows.iter().find(|w| w.name == name).unwrap();
                    }
                    Action::Reject => return false,
                    Action::Accept => return true,
                }
            }
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

fn test_part_with_workflow(part: &Part, workflow: &Workflow) -> Action {
    for rule in &workflow.rules {
        match rule {
            Rule::Test {
                rating,
                compare,
                action,
            } => {
                if compare.test_with(part.get_from_str(rating)) {
                    return action.clone();
                }
            }

            Rule::Action(action) => return action.clone(),
        }
    }

    unreachable!("Unable to complete the workflow");
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
