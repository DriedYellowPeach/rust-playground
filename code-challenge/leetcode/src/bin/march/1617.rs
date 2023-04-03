use std::collections::HashMap;

use leetcode::helper::builder::{graph_from_edges, GraphType};

fn main() {
    unimplemented!();
}

// return depth
fn dfs(graph: &HashMap<i32, Vec<i32>>, root: i32, mask: &mut i32, diameter: &mut i32) -> i32 {
    *mask &= !(1 << root);
    let arounds = match graph.get(&root) {
        Some(v) => v,
        None => return 0,
    };

    let mut max_depth = 0;

    for v in arounds {
        if *mask & (1 << *v) != 0 {
            // seen
            let depth = 1 + dfs(graph, *v, mask, diameter);
            *diameter = std::cmp::max(*diameter, depth + max_depth);
            max_depth = std::cmp::max(max_depth, depth);
        }
    }

    max_depth
}

#[test]
fn test_bit_mask() {
    let mut mask = 15;
    assert_eq!(mask & !(1 << 3), 7);
}

#[test]
fn test_dfs_ii() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![2, 4]];
    let edges = edges.into_iter().map(|mut it| { it.iter_mut().map(|v| *v -= 1).last(); it} ).collect();
    println!("{edges:?}");
    let graph = graph_from_edges(edges, GraphType::Multi);
    let root = first_nonzero_bit(3);
    println!("{root}");
    let mut mask = 3;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");

}

#[test]
fn test_dfs() {
    let edges = vec![vec![1, 2], vec![2, 3], vec![2, 4]];
    let edges = edges.into_iter().map(|mut it| { it.iter_mut().map(|v| *v -= 1).last(); it} ).collect();
    println!("{edges:?}");
    let graph = graph_from_edges(edges, GraphType::Multi);
    let root = first_nonzero_bit(15);
    println!("{root}");
    let mut mask = 15;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");

    let root = first_nonzero_bit(7);
    println!("{root}");
    let mut mask = 7;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");

    let root = first_nonzero_bit(13);
    println!("{root}");
    let mut mask = 13;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");

    let root = first_nonzero_bit(14);
    println!("{root}");
    let mut mask = 14;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");

    let root = first_nonzero_bit(11);
    println!("{root}");
    let mut mask = 11;
    let mut diameter = 0;
    dfs(&graph, root, &mut mask, &mut diameter);
    println!("mask: {mask}, diameter: {diameter}");
}

fn first_nonzero_bit(n: i32) -> i32 {
    let mut cnt = -1;
    let mut n = n;
    while n != 0 {
        n >>= 1;
        cnt += 1;
    }
    cnt
}

#[test]
fn test_first_nonzero_bit() {
    assert_eq!(first_nonzero_bit(0), -1);
    assert_eq!(first_nonzero_bit(1), 0);
    assert_eq!(first_nonzero_bit(2), 1);
    assert_eq!(first_nonzero_bit(3), 1);
    assert_eq!(first_nonzero_bit(4), 2);
}

fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
    let edges = edges.into_iter().map(|mut it| { it.iter_mut().map(|v| *v -= 1).last(); it} ).collect();
    let graph = graph_from_edges(edges, GraphType::Multi);
    let mut out = vec![0; (n - 1) as usize];
    for i in 1..(1 << n) {
        let root = first_nonzero_bit(i);
        let mut mask = i;
        let mut diameter = 0;
        dfs(&graph, root, &mut mask, &mut diameter);
        if mask == 0 && diameter >= 1 {
            out[(diameter - 1) as usize] += 1;
        }
    }
    out
}

#[test]
fn test_count_diamter() {
    let n = 4;
    let edges = vec![vec![1, 2], vec![2, 3], vec![2, 4]];
    let out = count_subgraphs_for_each_diameter(n, edges);
    println!("{out:?}");
}

#[test]
fn test_modify_vector() {
    let mut v = vec![1, 2, 3, 4];
    for i in v.iter_mut() {
        *i += 1;
    }
    println!("v[0]");
}
