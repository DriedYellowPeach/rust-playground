use std::mem;


fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    // loop roads, count order of every vertex
    // write down the first and 2nd max order
    // if thest two vertex connected, out -= 1
    let mut orders = vec![0; n as usize];
    let mut adj = vec![vec![false; n as usize]; n as usize];

    for r in roads.iter() {
        let v1 = r[0] as usize;
        let v2 = r[1] as usize;
        orders[v1] += 1;
        orders[v2] += 1;
        adj[v1][v2] = true;
        adj[v2][v1] = true;
    }

    // println!("{:?}", orders);
    // println!("{:?}", adj);

    let mut top_ones = vec![];
    let mut top_twos = vec![];

    for (i, &o) in orders.iter().enumerate() {
        let top1 = if top_ones.is_empty() { 0 } else { orders[top_ones[0]] };
        let top2 = if top_twos.is_empty() { 0 } else { orders[top_twos[0]] };

        if o > top1 {
            mem::swap(&mut top_ones, &mut top_twos);
            top_ones.clear();
            top_ones.push(i);
        } else if o == top1 {
            top_ones.push(i);
        } else if o < top1 && o > top2 {
            top_twos.clear();
            top_twos.push(i);
        } else if o == top2 {
            top_twos.push(i);
        }
    }

    // println!("{:?}", top_ones);
    // println!("{:?}", top_twos);
    let mut max_out = 0;

    if top_ones.len() == 1 {
        let first = orders[top_ones[0]];
        for idx in top_twos {
            let second = orders[idx];
            let connected = adj[top_ones[0]][idx];
            let order_sum = first + second - if connected { 1 } else { 0 };
            if order_sum > max_out {
                max_out = order_sum;
            }
        }
    } else {
        for i in 0..top_ones.len() {
            for j in i + 1..top_ones.len() {
                let connected = adj[top_ones[i]][top_ones[j]];
                let order_sum = orders[top_ones[i]] + orders[top_ones[j]] - if connected { 1 } else { 0 };
                if order_sum > max_out {
                    max_out = order_sum;
                }
            }
        
        }
    }

    max_out
}

#[test]
fn test_max() {
    let orders = [5, 2, 3, 4, 4, 4, 1, 5, 5];
    let mut top_ones = vec![];
    let mut top_twos = vec![];

    for (i, &o) in orders.iter().enumerate() {
        // let first = orders[first_idx[0]];
        // let second = orders[second_idx[0]];
        let top1 = if top_ones.is_empty() { 0 } else { orders[top_ones[0]] };
        let top2 = if top_twos.is_empty() { 0 } else { orders[top_twos[0]] };

        if o > top1 {
            top_ones.clear();
            top_ones.push(i);
        } else if o == top1 {
            top_ones.push(i);
        } else if o < top1 && o > top2 {
            top_twos.clear();
            top_twos.push(i);
        } else if o == top2 {
            top_twos.push(i);
        }
    }

    println!("{top_ones:?}");
    println!("{top_twos:?}");

    top_ones.iter().inspect(|idx| print!("{} ", orders[**idx])).last();
    println!();
    top_twos.iter().inspect(|idx| print!("{} ", orders[**idx])).last();
}

#[test]
fn test_rank() {
    let n = 4;
    let roads = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3]];
    assert_eq!(maximal_network_rank(n, roads), 4);

    let n = 5;
    let roads = vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3], vec![2, 3], vec![2, 4]];
    assert_eq!(maximal_network_rank(n, roads), 5);

    let n = 2;
    let roads = vec![vec![1, 0]];
    assert_eq!(maximal_network_rank(n, roads), 1);

    let n = 27;
    let roads = [[4,21],[15,20],[21,12],[0,25],[23,15],[8,0],[1,15],[3,14],[10,6],[13,19],[13,14],[10,9],[9,1],[4,18],[20,21],[8,20],[7,5],[5,20],[26,16],[13,15],[5,9],[21,1],[1,22],[5,15],[2,25],[0,24],[6,4],[5,17],[6,20],[4,16],[23,7],[2,3],[9,17],[4,13],[12,1],[1,25],[7,18],[5,18],[11,1],[6,15],[19,17],[15,24],[4,9],[0,22],[5,23],[8,2],[7,1],[19,16],[13,0],[6,3],[17,23],[12,24],[17,7],[18,14],[5,16],[7,13],[3,23],[22,20],[25,26],[5,24]];
    let roads = roads.iter().map(|r| vec![r[0], r[1]]).collect();
    assert_eq!(maximal_network_rank(n, roads), 17);

}
