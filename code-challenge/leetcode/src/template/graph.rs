use crate::helper::builder;


pub fn bfs() {
    let edges = vec![vec![0, 1]];
    println!("bfs");
    let graph = builder::graph_from_edges(edges, builder::GraphType::Multi);
    println!("{graph:?}");
}
