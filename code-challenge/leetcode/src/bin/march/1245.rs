use std::collections::{ HashMap, HashSet };

use leetcode::helper::builder::{ graph_from_edges, GraphType};

fn main() {
    unimplemented!();
}

// dfs iter the graph from vertex, return the vertex cnt
fn dfs(graph: &HashMap<i32, Vec<i32>>, vertex: i32, out_max: &mut i32, seen: &mut HashSet<i32>) -> i32 {
    seen.insert(vertex);
    let arounds = match graph.get(&vertex) {
        Some(v) => v,
        None => return 1,
    };

    let mut max_depth = 0;
    for v in arounds {
        if seen.get(v).is_some() {
            continue
        }
        let depth = dfs(graph, *v, out_max, seen);
        // println!("v:{v} has depth: {depth}");
        *out_max = std::cmp::max(*out_max, depth + max_depth); /* if depth is the first or snd */
        max_depth = std::cmp::max(max_depth, depth);
    }

    max_depth + 1
}

#[allow(dead_code)]
fn tree_diameter(edges: Vec<Vec<i32>>) -> i32 {
    let graph = graph_from_edges(edges, GraphType::Multi);
    let mut seen = HashSet::<i32>::new();
    let mut out_max = 0;
    dfs(&graph, 0, &mut out_max, &mut seen);

    out_max
}


#[test]
fn test_tree_diameter() {
    let edges = vec![vec![0, 1], vec![0, 2]];
    assert_eq!(tree_diameter(edges), 2);
    
    println!("===========");

    let edges = vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![1, 4], vec![4, 5]];
    assert_eq!(tree_diameter(edges), 4);
}
