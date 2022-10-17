use std::collections::HashMap;
use std::hash::Hash;

pub fn graph_from_edges<T>(edges: Vec<Vec<T>>) -> HashMap<T, Vec<T>>
where T: Eq + Hash + Copy,
{
    let mut g = HashMap::new();
    for e in edges {
        let v = g.entry(e[0]).or_insert(Vec::new());
        v.push(e[1]);
    }
    g
}

#[test]
fn test_graph_from_edges() {
    let v = vec![vec![0, 1], vec![1, 2], vec![0, 2]];
    let g = graph_from_edges(v);
    assert_eq!(g.get(&0).unwrap(), &vec![1, 2]);
    assert_eq!(g.get(&1).unwrap(), &vec![2]);
}