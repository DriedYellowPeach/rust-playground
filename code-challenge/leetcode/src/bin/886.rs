
fn main() {

}

use std::collections::HashMap;

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

    return !ctx.has_loop
}

fn dfs(ctx: &mut context, node: i32, previous: i32, depth: i32) {
    ctx.seen.insert(node, depth);
    if let Some(childs) = ctx.relations.get(&node) {
        for c in childs {
            if let Some(v) = ctx.seen.get(c) {
                let previous_depth = ctx.seen.get(&previous).unwrap();
                if *c != previous && (*previous_depth - depth) % 2 == 0{
                    // println!("{}, {}, {}, {}", ctx.previous, node, c, *v);
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





#[test]
fn test_possible_bipartition() {
    let mut v1 = vec![vec![1,2],
                                  vec![1, 3], 
                                  vec![2, 4]];
    // println!("{}", possible_bipartition(4, v));
    assert_eq!(possible_bipartition(4, v1), true);
    let v2 = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]];
    assert_eq!(possible_bipartition(4, v2), false);

}

#[test]
fn test_dislike_to_relations() {
    // let mut v = vec![vec![0,1],
    //                               vec![1, 2], 
    //                               vec![1, 0]];
    let mut v1 = vec![vec![1,2],
                                  vec![1, 3], 
                                  vec![2, 4]];
    println!("{:?}", dislike_to_relations(v1));
}