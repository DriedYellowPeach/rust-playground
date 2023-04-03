fn main() {
    unimplemented!();
}

use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::Hash;

#[allow(dead_code)]
fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
    if edges.len() != (n - 1) as usize {
        return false;
    }

    if n == 1 {
        return true;
    }

    let graph = edges_to_graph(edges);
    // println!("{graph:?}");
    let mut seen = vec![0; n as usize];
    let mut q = VecDeque::new();
    q.push_back(0);
    seen[0] = 1;

    // head first element, map[head] all the vertex, iter all vertex, enter queue
    // vertex seen, loop, return false
    // else push_back vertex
    while !q.is_empty() {
        let head = q.pop_front().unwrap();
        // println!("{head}");
        let neighbors = match graph.get(&head) {
            Some(n) => n,
            None => return false,
        };
        for n in neighbors {
            if seen[*n as usize] == 0 {
                q.push_back(*n);
                seen[*n as usize] = 1;
            }
        }
    }

    // check if seen has zero, return false
    return seen.iter().all(|n| n == &1);
}

#[allow(dead_code)]
fn edges_to_graph<T>(edges: Vec<Vec<T>>) -> HashMap<T, HashSet<T>>
where
    T: Hash + PartialEq + Copy + Eq,
{
    let mut graph = HashMap::new();
    for pair in edges {
        if pair.len() != 2 {
            panic!("invalid edges format");
        }
        let (vertex1, vertex2) = (pair[0], pair[1]);
        let ent = graph.entry(vertex1).or_insert_with(HashSet::new);
        ent.insert(vertex2);
        graph
            .entry(vertex2)
            .or_insert_with(HashSet::new)
            .insert(vertex1);
    }

    graph
}

#[test]
fn test_edges_to_graph() {
    let edges = [[0, 1], [1, 2], [2, 0], [2, 1], [3, 4]];
    let graph = edges_to_graph(
        edges
            .into_iter()
            .map(|e| vec![e[0], e[1]])
            .collect::<Vec<Vec<i32>>>(),
    );
    println!("{graph:?}");
}

#[test]
fn test_valid_tree() {
    let edges = [[0, 1], [0, 4], [1, 4], [2, 3], [1, 2]];
    assert_eq!(
        valid_tree(5, edges.into_iter().map(|e| vec![e[0], e[1]]).collect()),
        false
    );
}
