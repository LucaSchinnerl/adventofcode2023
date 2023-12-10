use petgraph::graph::DiGraph;
use petgraph::prelude::*;
use petgraph::visit::Bfs;
use petgraph::Direction;
use std::collections::HashMap;

use std::usize;

#[aoc::main(10)]
fn main(input: &str) -> (u32, u32) {
    (solution(input), 0)
}

fn solution(input: &str) -> u32 {
    let (graph, starting_node) = create_node_edge_pairs(input);

    let mut bfs = Bfs::new(&graph, starting_node);
    let mut distance = 0;

    while let Some(_next_node) = bfs.next(&graph) {
        distance += 1;
    }
    distance / 2
}

fn create_node_edge_pairs(input: &str) -> (DiGraph<(usize, usize), ()>, NodeIndex) {
    let x_bound = input.lines().next().unwrap().len();
    let y_bound = input.lines().count();
    let mut graph: DiGraph<(usize, usize), ()> = DiGraph::new();
    let mut node_indices = HashMap::new();
    let mut starting_node: NodeIndex = NodeIndex::new(0);
    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            let index = graph.add_node((x, y));
            if c == 'S' {
                starting_node = index;
            }
            node_indices.insert((x, y), index);
        });
    });

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c == '.' {
                return;
            }
            let connections = get_connections(c, x, y);
            connections.iter().for_each(|(x_new, y_new)| {
                if check_bounds(*x_new, *y_new, x_bound, y_bound) {
                    let target_index = node_indices.get(&(*x_new, *y_new));
                    // If target_index is an error print the index and the coordinates
                    let target_index = match target_index {
                        Some(index) => *index,
                        None => {
                            return;
                        }
                    };
                    graph.add_edge(node_indices[&(x, y)], target_index, ());
                }
            });
        });
    });

    let pointing_nodes = graph
        .neighbors_directed(starting_node, Direction::Incoming)
        .collect::<Vec<NodeIndex>>();

    pointing_nodes.iter().for_each(|node_index| {
        graph.add_edge(starting_node, *node_index, ());
    });
    (graph, starting_node)
}

fn get_connections(c: char, x: usize, y: usize) -> Vec<(usize, usize)> {
    match c {
        '|' => vec![(x, y + 1), (x, y.saturating_sub(1))],
        '-' => vec![(x + 1, y), (x.saturating_sub(1), y)],
        'L' => vec![(x, y.saturating_sub(1)), (x + 1, y)],
        'J' => vec![(x, y.saturating_sub(1)), (x.saturating_sub(1), y)],
        '7' => vec![(x, y + 1), (x.saturating_sub(1), y)],
        'F' => vec![(x, y + 1), (x + 1, y)],
        '.' => vec![],
        'S' => vec![],
        _ => {
            println!("{}", c);
            panic!("Invalid character")
        }
    }
}

fn check_bounds(x: usize, y: usize, x_bound: usize, y_bound: usize) -> bool {
    x < x_bound && y < y_bound
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_connections() {
        assert_eq!(get_connections('|', 10, 10), vec![(10, 11), (10, 9)]);
        assert_eq!(get_connections('-', 10, 10), vec![(11, 10), (9, 10)]);
        assert_eq!(get_connections('L', 10, 10), vec![(10, 9), (11, 10)]);
        assert_eq!(get_connections('J', 10, 10), vec![(10, 9), (9, 10)]);
        assert_eq!(get_connections('7', 10, 10), vec![(10, 11), (9, 10)]);
        assert_eq!(get_connections('F', 10, 10), vec![(10, 11), (11, 10)]);
        assert_eq!(get_connections('.', 10, 10), vec![]);
    }

    #[test]
    fn test_first_example() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(solution(input), 4);
    }

    #[test]
    fn test_second_example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solution(input), 8);
    }
}
