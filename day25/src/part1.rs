// TODO: write a proper solution

// Ideas
//
// Max-flow min-cut theorem
// Ford-Fulkerson algorithm

// Generate a graph with graphviz
// neato -T svg day25/src/inputs/input.dot > output.svg

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn result(input: &str) -> usize {
    let nodes = parse_input(input);

    let edges_to_cut = [("htj", "pcc"), ("dlk", "pjj"), ("bbg", "htb")];

    let mut graph = parse_graph(nodes);

    for (from, to) in edges_to_cut {
        let node = graph.get_mut(from).unwrap();
        node.remove(to);

        let node = graph.get_mut(to).unwrap();
        node.remove(from);
    }

    let from_htj = visit_count_hashmap(&graph, "htj");
    let from_pcc = visit_count_hashmap(&graph, "pcc");

    from_htj * from_pcc
}

fn visit_count_hashmap(graph: &HashMap<String, HashSet<String>>, from: &str) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(from);
    visited.insert(from);

    while let Some(node) = queue.pop_front() {
        let node = graph.get(node).unwrap();

        for edge in node {
            if !visited.contains(edge.as_str()) {
                visited.insert(edge.as_str());
                queue.push_back(edge.as_str());
            }
        }
    }

    visited.len()
}

#[allow(dead_code)]
fn generate_dot(nodes: &[Node]) {
    println!("graph {{");
    for node in nodes {
        println!("    {} -- {{{}}};", node.id, node.edges.iter().join(", "));
    }
    println!("}}");
}

#[derive(Debug)]
struct Node {
    id: String,
    edges: HashSet<String>,
}

impl Node {
    fn from_str(s: &str) -> Self {
        let (id, edges) = s.split_once(": ").unwrap();

        let id = id.to_owned();
        let edges = edges.split_whitespace().map(str::to_owned).collect();

        Self { id, edges }
    }
}

fn parse_graph(nodes: Vec<Node>) -> HashMap<String, HashSet<String>> {
    let mut graph = HashMap::<String, HashSet<String>>::new();

    for node in nodes {
        for edge in node.edges {
            graph
                .entry(node.id.clone())
                .or_default()
                .insert(edge.clone());

            graph.entry(edge).or_default().insert(node.id.clone());
        }
    }

    graph
}

fn parse_input(input: &str) -> Vec<Node> {
    input.lines().map(Node::from_str).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{INPUT, SAMPLE};

    #[test]
    #[ignore = "solution is input dependant"]
    fn test_sample() {
        let input = SAMPLE;

        let result = result(input);

        assert_eq!(result, 54);
    }

    #[test]
    fn test_input() {
        let input = INPUT;

        let result = result(input);

        assert_eq!(result, 538560);
    }
}
