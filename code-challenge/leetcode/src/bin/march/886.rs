
fn main() {

}

use std::collections::{HashMap, VecDeque};

fn dislike_to_relations(dislike: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut ret = HashMap::new();
    for relation in dislike {
       let val = ret.entry(relation[0]).or_insert(Vec::new());
       val.push(relation[1]);

       let val = ret.entry(relation[1]).or_insert(Vec::new());
       val.push(relation[0]);
    }
    ret
}

struct context<'a> {
    has_loop: bool,
    relations: &'a HashMap<i32, Vec<i32>>,
    seen: &'a mut HashMap<i32, i32>
}

fn possible_bipartition(n: i32, dislike: Vec<Vec<i32>>) -> bool {
    if dislike.len() < 2 {
        return true;
    }

    let mut seen = HashMap::new();
    let root = dislike[0][0];
    let mut relations = dislike_to_relations(dislike);
    let ctx = &mut context{ has_loop: false, relations: &relations, seen: &mut seen};

    for i in 1..n+1 {
        if !ctx.seen.contains_key(&i) {
            dfs(ctx, i, i, 0);
        }

        if ctx.has_loop {
            break
        }
    }
    //println!("{}", ctx.has_loop);

    return !ctx.has_loop
}

fn dfs(ctx: &mut context, node: i32, previous: i32, depth: i32) {
    ctx.seen.insert(node, depth);
    if let Some(childs) = ctx.relations.get(&node) {
        for c in childs {
            if let Some(v) = ctx.seen.get(c) {
                //let previous_depth = ctx.seen.get(&previous).unwrap();
                //let child_depth = ctx.seen.get(c).unwrap();
                if *c != previous && (depth - v) % 2 == 0{
                    //println!("{}, {}, {}, {}, {}", previous, node, c, *v, depth);
                    ctx.has_loop = true;
                    return
                }
            } else {
                dfs(ctx, *c, node, depth+1);
            }
            if ctx.has_loop {
                return
            }
        }
    }
}

fn possible_bipartition_old(n: i32, dislike: Vec<Vec<i32>>) -> bool {
    if dislike.len() < 2 {
        return true;
    }

    use std::collections::VecDeque;
    let mut q = VecDeque::new();
    let mut partion_result = HashMap::new();
    let mut relations = HashMap::new();

    let root = dislike[0][0];
    let mut grp_tag = 0;

    relations = dislike_to_relations(dislike);

    // init from root of the relations graph
    partion_result.insert(root, grp_tag);
    grp_tag = 1;
    let grp = relations.get(&root).unwrap().clone();
    for v in &grp {
        partion_result.insert(*v, grp_tag);
    }
    q.push_back(grp);

    
    while q.len() != 0 {
        let mut next_grp = Vec::new();
        let grp = q.pop_front().unwrap();
        for p in &grp {
            // already painted, check compatible
            match partion_result.get(p) {
                Some(v) => { 
                    if *v != grp_tag {
                        // println!("{:?}", relations);
                        // println!("{:?}", partion_result);
                        // println!("{}:{}", *p, *v);
                        return false
                    }
                }
                // haven't painted, paint with reverse tag
                None => {
                    partion_result.entry(*p).or_insert(grp_tag);
                }
            }
            if let Some(p_dislike) = relations.get(p) {
                next_grp.extend(p_dislike);
            }
        }
        if partion_result.len() == n as usize {
            return true
        }

        q.push_back(next_grp);
        grp_tag = if grp_tag == 0 { 1 } else { 0 };
    }

    true
}

struct Env<'a> {
    graph: &'a HashMap<i32, Vec<i32>>,
    colors: &'a mut Vec<i32>
}

fn coloring(n: i32, dislike: Vec<Vec<i32>>) -> bool {
    use leetcode::helper::builder;
    let graph = builder::graph_from_edges(dislike, builder::GraphType::Multi);
    let mut colors = vec![0; n as usize + 1];
    let env = &mut Env{ graph: &graph, colors: &mut colors};
    
    for i in 1..n+1 {
        if env.colors[i as usize] == 0 && !do_color(env, i, 1) {
            return false
        }
    }

    true
}

fn do_color(env: &mut Env, node: i32, color: i32) -> bool {
    env.colors[node as usize] = color;
    if let Some(kids) = env.graph.get(&node) {
        for k in kids {
            let k_color = if color == 1 { 2 } else { 1 };
            if env.colors[*k as usize] == color || env.colors[*k as usize] == 0 && !do_color(env, *k, k_color) {
                // println!("{} {}", node, *k);
                return false
            }
        }
    }
    true
}

#[test]
fn test_coloring() {
    let mut v1 = vec![vec![1,2],
                                  vec![1, 3], 
                                  vec![2, 4]];
    // println!("{}", possible_bipartition(4, v));
    assert_eq!(coloring(4, v1), true);
    let v2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    assert_eq!(coloring(5, v2), false);
    let v3 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4]];
    assert_eq!(coloring(4, v3), true);

    let arr = [[21,47],[4,41],[2,41],[36,42],[32,45],[26,28],[32,44],[5,41],[29,44],[10,46],[1,6],[7,42],[46,49],[17,46],[32,35],[11,48],[37,48],[37,43],[8,41],[16,22],[41,43],[11,27],[22,44],[22,28],[18,37],[5,11],[18,46],[22,48],[1,17],[2,32],[21,37],[7,22],[23,41],[30,39],[6,41],[10,22],[36,41],[22,25],[1,12],[2,11],[45,46],[2,22],[1,38],[47,50],[11,15],[2,37],[1,43],[30,45],[4,32],[28,37],[1,21],[23,37],[5,37],[29,40],[6,42],[3,11],[40,42],[26,49],[41,50],[13,41],[20,47],[15,26],[47,49],[5,30],[4,42],[10,30],[6,29],[20,42],[4,37],[28,42],[1,16],[8,32],[16,29],[31,47],[15,47],[1,5],[7,37],[14,47],[30,48],[1,10],[26,43],[15,46],[42,45],[18,42],[25,42],[38,41],[32,39],[6,30],[29,33],[34,37],[26,38],[3,22],[18,47],[42,48],[22,49],[26,34],[22,36],[29,36],[11,25],[41,44],[6,46],[13,22],[11,16],[10,37],[42,43],[12,32],[1,48],[26,40],[22,50],[17,26],[4,22],[11,14],[26,39],[7,11],[23,26],[1,20],[32,33],[30,33],[1,25],[2,30],[2,46],[26,45],[47,48],[5,29],[3,37],[22,34],[20,22],[9,47],[1,4],[36,46],[30,49],[1,9],[3,26],[25,41],[14,29],[1,35],[23,42],[21,32],[24,46],[3,32],[9,42],[33,37],[7,30],[29,45],[27,30],[1,7],[33,42],[17,47],[12,47],[19,41],[3,42],[24,26],[20,29],[11,23],[22,40],[9,37],[31,32],[23,46],[11,38],[27,29],[17,37],[23,30],[14,42],[28,30],[29,31],[1,8],[1,36],[42,50],[21,41],[11,18],[39,41],[32,34],[6,37],[30,38],[21,46],[16,37],[22,24],[17,32],[23,29],[3,30],[8,30],[41,48],[1,39],[8,47],[30,44],[9,46],[22,45],[7,26],[35,42],[1,27],[17,30],[20,46],[18,29],[3,29],[4,30],[3,46]];
    fn arr_to_vec(arr: [[i32;2]; 196]) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        for e in arr {
            v.push(Vec::from_iter(e));
        }

        v
    }
    assert_eq!(coloring(50, arr_to_vec(arr)), true);
}

#[test]
fn test_possible_bipartition() {
    let v1 = vec![vec![1,2],
                                  vec![1, 3], 
                                  vec![2, 4]];
    // println!("{}", possible_bipartition(4, v));
    assert_eq!(possible_bipartition(4, v1), true);
    let v2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    assert_eq!(possible_bipartition(4, v2), false);
    let v3 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4]];
    assert_eq!(possible_bipartition(4, v3), true);

    let arr = [[21,47],[4,41],[2,41],[36,42],[32,45],[26,28],[32,44],[5,41],[29,44],[10,46],[1,6],[7,42],[46,49],[17,46],[32,35],[11,48],[37,48],[37,43],[8,41],[16,22],[41,43],[11,27],[22,44],[22,28],[18,37],[5,11],[18,46],[22,48],[1,17],[2,32],[21,37],[7,22],[23,41],[30,39],[6,41],[10,22],[36,41],[22,25],[1,12],[2,11],[45,46],[2,22],[1,38],[47,50],[11,15],[2,37],[1,43],[30,45],[4,32],[28,37],[1,21],[23,37],[5,37],[29,40],[6,42],[3,11],[40,42],[26,49],[41,50],[13,41],[20,47],[15,26],[47,49],[5,30],[4,42],[10,30],[6,29],[20,42],[4,37],[28,42],[1,16],[8,32],[16,29],[31,47],[15,47],[1,5],[7,37],[14,47],[30,48],[1,10],[26,43],[15,46],[42,45],[18,42],[25,42],[38,41],[32,39],[6,30],[29,33],[34,37],[26,38],[3,22],[18,47],[42,48],[22,49],[26,34],[22,36],[29,36],[11,25],[41,44],[6,46],[13,22],[11,16],[10,37],[42,43],[12,32],[1,48],[26,40],[22,50],[17,26],[4,22],[11,14],[26,39],[7,11],[23,26],[1,20],[32,33],[30,33],[1,25],[2,30],[2,46],[26,45],[47,48],[5,29],[3,37],[22,34],[20,22],[9,47],[1,4],[36,46],[30,49],[1,9],[3,26],[25,41],[14,29],[1,35],[23,42],[21,32],[24,46],[3,32],[9,42],[33,37],[7,30],[29,45],[27,30],[1,7],[33,42],[17,47],[12,47],[19,41],[3,42],[24,26],[20,29],[11,23],[22,40],[9,37],[31,32],[23,46],[11,38],[27,29],[17,37],[23,30],[14,42],[28,30],[29,31],[1,8],[1,36],[42,50],[21,41],[11,18],[39,41],[32,34],[6,37],[30,38],[21,46],[16,37],[22,24],[17,32],[23,29],[3,30],[8,30],[41,48],[1,39],[8,47],[30,44],[9,46],[22,45],[7,26],[35,42],[1,27],[17,30],[20,46],[18,29],[3,29],[4,30],[3,46]];

}

fn coloring_with_queue(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
    use leetcode::helper::builder;
    let graph = builder::graph_from_edges(dislikes, builder::GraphType::Multi);
    let mut q = VecDeque::new();
    let mut colors = vec![0; n as usize + 1];

    for i in 1..n+1 {
        if colors[i as usize] == 0 {
            q.push_back(i);
            colors[i as usize] = 1;
            while q.len() != 0 {
                let p = q.pop_front().unwrap();

                if let Some(kids) = graph.get(&p) {
                    for k in kids {
                        if colors[*k as usize] == colors[p as usize] {
                            println!("{} {} {:?}", p, k, colors);
                            return false
                        }

                        if colors[*k as usize] == 0 {
                            colors[*k as usize] = if colors[p as usize] == 1 { 2 } else { 1 };
                            q.push_back(*k);
                        }
                    }
                }
            }
        }
    }

    true
}


#[test]
fn test_coloring_with_queue() {
    let mut v1 = vec![vec![1,2],
                                  vec![1, 3], 
                                  vec![2, 4]];
    // println!("{}", possible_bipartition(4, v));
    assert_eq!(coloring_with_queue(4, v1), true);
    let v2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    assert_eq!(coloring_with_queue(5, v2), false);
    let v3 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4]];
    assert_eq!(coloring_with_queue(4, v3), true);

    let arr = [[21,47],[4,41],[2,41],[36,42],[32,45],[26,28],[32,44],[5,41],[29,44],[10,46],[1,6],[7,42],[46,49],[17,46],[32,35],[11,48],[37,48],[37,43],[8,41],[16,22],[41,43],[11,27],[22,44],[22,28],[18,37],[5,11],[18,46],[22,48],[1,17],[2,32],[21,37],[7,22],[23,41],[30,39],[6,41],[10,22],[36,41],[22,25],[1,12],[2,11],[45,46],[2,22],[1,38],[47,50],[11,15],[2,37],[1,43],[30,45],[4,32],[28,37],[1,21],[23,37],[5,37],[29,40],[6,42],[3,11],[40,42],[26,49],[41,50],[13,41],[20,47],[15,26],[47,49],[5,30],[4,42],[10,30],[6,29],[20,42],[4,37],[28,42],[1,16],[8,32],[16,29],[31,47],[15,47],[1,5],[7,37],[14,47],[30,48],[1,10],[26,43],[15,46],[42,45],[18,42],[25,42],[38,41],[32,39],[6,30],[29,33],[34,37],[26,38],[3,22],[18,47],[42,48],[22,49],[26,34],[22,36],[29,36],[11,25],[41,44],[6,46],[13,22],[11,16],[10,37],[42,43],[12,32],[1,48],[26,40],[22,50],[17,26],[4,22],[11,14],[26,39],[7,11],[23,26],[1,20],[32,33],[30,33],[1,25],[2,30],[2,46],[26,45],[47,48],[5,29],[3,37],[22,34],[20,22],[9,47],[1,4],[36,46],[30,49],[1,9],[3,26],[25,41],[14,29],[1,35],[23,42],[21,32],[24,46],[3,32],[9,42],[33,37],[7,30],[29,45],[27,30],[1,7],[33,42],[17,47],[12,47],[19,41],[3,42],[24,26],[20,29],[11,23],[22,40],[9,37],[31,32],[23,46],[11,38],[27,29],[17,37],[23,30],[14,42],[28,30],[29,31],[1,8],[1,36],[42,50],[21,41],[11,18],[39,41],[32,34],[6,37],[30,38],[21,46],[16,37],[22,24],[17,32],[23,29],[3,30],[8,30],[41,48],[1,39],[8,47],[30,44],[9,46],[22,45],[7,26],[35,42],[1,27],[17,30],[20,46],[18,29],[3,29],[4,30],[3,46]];
    fn arr_to_vec(arr: [[i32;2]; 196]) -> Vec<Vec<i32>> {
        let mut v = Vec::new();
        for e in arr {
            v.push(Vec::from_iter(e));
        }

        v
    }
    assert_eq!(coloring_with_queue(50, arr_to_vec(arr)), true);
}

#[test]
fn test_iter_without_ref() {
    let n = 5;
    let v = vec![1; n];
    for &y in &v {
        println!("{}", v[y as usize]);
    }
}