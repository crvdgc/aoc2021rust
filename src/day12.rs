use std::collections::{HashMap, HashSet};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

pub fn part1(input: &str) -> u32 {
    let graph = parse_edges(&input);
    println!("{:?}", graph);
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    search_path(&graph, &mut visited, "start")
}

pub fn part2(input: &str) -> u32 {
    let graph = parse_edges(&input);
    println!("{:?}", graph);
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("start");
    search_path2(&graph, &mut visited, "start", true)
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

fn is_small(cave: &str) -> bool {
    cave.chars().next().unwrap().is_lowercase()
}

fn search_path<'a>(graph: &'a Graph<'a>, visited: &mut HashSet<&'a str>, from: &str) -> u32 {
    if from == "end" {
        1
    }
    else if let Some(neighbors) = graph.get(from) {
        let mut paths = 0;
        for to in neighbors.iter() {
            if is_small(to) {
                if visited.insert(to) { // not visited
                    paths += search_path(graph, visited, to);
                    visited.remove(to);
                }
            } else {
                paths += search_path(graph, visited, to);
            }
        }
        paths
    } else {
        panic!("no from")
    }
}

fn search_path2<'a>(graph: &'a Graph<'a>, visited: &mut HashSet<&'a str>, from: &str, can_twice: bool) -> u32 {
    if from == "end" {
        1
    }
    else if let Some(neighbors) = graph.get(from) {
        let mut paths = 0;
        for to in neighbors.iter() {
            if *to == "start" {
                continue;
            } else if is_small(to) {
                if visited.insert(to) { // not visited
                    paths += search_path2(graph, visited, to, can_twice);
                    // restore small visits
                    visited.remove(to);
                } else if can_twice {
                    // visit this twice
                    paths += search_path2(graph, visited, to, false);
                }
            } else {
                paths += search_path2(graph, visited, to, can_twice);
            }
        }
        paths
    } else {
        panic!("no from")
    }
}
