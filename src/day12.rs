use std::collections::HashMap;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn part1(input: &str) -> u32 {
    let graph = parse_edges(&input);
    println!("{:?}", graph);
    0
}

fn is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn parse_edges(input: &str) -> Graph {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut vertexs = line.split('-');
        let start = vertexs.next().unwrap();
        let end = vertexs.next().unwrap();
        if let Some(neighbors) = graph.get_mut(start) {
            neighbors.push(end);
        } else {
            let mut neighbors = Vec::new();
            neighbors.push(end);
            graph.insert(start, neighbors);
        }
        if let Some(neighbors) = graph.get_mut(end) {
            neighbors.push(start);
        } else {
            let mut neighbors = Vec::new();
            neighbors.push(start);
            graph.insert(end, neighbors);
        }
    }
    graph
}

