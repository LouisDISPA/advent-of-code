#[cfg(test)]
mod tests;

use std::collections::HashMap;

type Input = HashMap<String, Vec<String>>;

pub fn add_connection_to_node(graph: &mut Input, a: &str, b: &str) {
    if let Some(node_a) = graph.get_mut(a) {
        if !node_a.iter().any(|conn| conn == b) {
            node_a.push(b.to_owned());
        }
    } else {
        graph.insert(a.to_owned(), vec![b.to_owned()]);
    }

    if let Some(node_b) = graph.get_mut(b) {
        if !node_b.iter().any(|conn| conn == a) {
            node_b.push(a.to_owned());
        }
    } else {
        graph.insert(b.to_owned(), vec![a.to_owned()]);
    }
}

pub fn parse_input(text: &str) -> Input {
    let mut graph = HashMap::new();
    for line in text.lines() {
        if let Some((a, b)) = line.split_once('-') {
            add_connection_to_node(&mut graph, a, b);
        }
    }
    graph
}

pub fn solve_part1(input: Input) -> usize {
    let mut path = Vec::new();
    find_the_end(&input, "start", &mut path)
}

pub fn solve_part2(input: Input) -> usize {
    let mut path = Vec::new();
    find_the_end2(&input, "start", &mut path, false)
}

pub fn find_the_end<'a>(graph: &'a Input, node: &'a str, path: &mut Vec<&'a str>) -> usize {
    if node == "end" {
        // println!("{}", path.join(" -> "));
        return 1;
    }
    path.push(node);
    let mut sum = 0;
    for node in graph.get(node).unwrap() {
        if &node.to_uppercase() == node || !path.iter().any(|path_node| path_node == node) {
            sum += find_the_end(graph, node, path);
        }
    }
    path.pop();
    sum
}

pub fn find_the_end2<'a>(
    graph: &'a Input,
    node: &'a str,
    path: &mut Vec<&'a str>,
    twice: bool,
) -> usize {
    if node == "end" {
        // println!("{}", path.join(" -> "));
        return 1;
    }
    let mut sum = 0;
    for node in graph.get(node).unwrap() {
        if node != "start" {
            if &node.to_uppercase() == node {
                path.push(node);
                sum += find_the_end2(graph, node, path, twice);
                path.pop();
            } else {
                let exist = path.iter().any(|path_node| path_node == node);
                if !exist {
                    path.push(node);
                    sum += find_the_end2(graph, node, path, twice);
                    path.pop();
                } else if !twice {
                    path.push(node);
                    sum += find_the_end2(graph, node, path, true);
                    path.pop();
                }
            }
        }
    }
    sum
}
