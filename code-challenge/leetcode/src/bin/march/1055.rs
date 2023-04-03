use std::collections::HashMap;


fn main() {
    println!("find the shortest way to construct string!");
}

fn shortest_way(source: String,  target: String) -> i32 {
    let source = source.as_bytes();
    let target = target.as_bytes();
    let mut dp: Vec<HashMap<usize, usize>> = vec![HashMap::new(); target.len()];

    // this may panic, handle it laster -> unwrap, find all idx from 0 start
    let idx = pos_after_start(source, -1, target[0]);
    dp[0].insert(idx.unwrap(), 1);

    for i in 1..dp.len() {
        // split prev and cur
        let (front, back) = dp.split_at_mut(i);
        let prev = front.last_mut().unwrap();
        let cur = &mut back[0];
        let first_idx = pos_after_start(source, -1, target[i]).unwrap();

        for (&start, &mut length) in prev {
            // situation ab|c|
            if let Some(&l) = cur.get(&first_idx) {
                cur.insert(first_idx, std::cmp::min(l, length+1));
            } else {
                cur.insert(first_idx, length+1);
            }
            // situation xxxy
            
            if let Some(n) = pos_after_start(source, start as i32, target[i]) {
                if let Some(&l) = cur.get(&n) {
                    cur.insert(n, std::cmp::min(length, l));
                } else {
                    cur.insert(n, length);
                }
            }
            // match pos_after_start(source, start as i32, target[i]) {
            //     Some(n) => {
            //         if let Some(&l) = cur.get(&n) {
            //             cur.insert(n, std::cmp::min(length, l));
            //         } else {
            //             cur.insert(n, length);
            //         }
            //     },
            //     None => {
            //         if let Some(&l) = cur.get(&first_idx) {
            //             cur.insert(first_idx, std::cmp::min(l, length+1));
            //         } else {
            //             cur.insert(first_idx, length + 1);
            //         }
            //     }
            // };
        }

    }

    // for (&idx, &mut length) in dp.last().unwrap() {
    //     if length < ret {
    //         ret 
    //     }    
    // }
    println!("{:?}", dp);
    let ret = dp.last().unwrap().iter().map(|(_, length)| length).min().unwrap();
    *ret as i32
}

fn pos_after_start(src: &[u8], start: i32, target: u8) -> Option<usize> {
    if start < 0 && start != -1 {
        return None
    }

    if start == -1 {
        return src.iter().position(|x| *x == target)
    }

    let start:usize = start as usize;

    if start >= src.len() - 1 {
        // start too much, can't find target
        return None
    }

    let position = src[start+1..].iter().position(|x| *x == target);
    match position {
        // may overflow, but the size won't be that much
        Some(idx) => { Some(idx + start + 1)},
        None => { None }
    }

}

#[test]
fn test_position_after_start() {
    let nums: Vec<u8> = vec![2, 1, 2, 4, 2];
    assert_eq!(pos_after_start(&nums, -1, 2), Some(0));
    assert_eq!(pos_after_start(&nums, 0, 2), Some(2));
    assert_eq!(pos_after_start(&nums, 1, 2), Some(2));
    assert_eq!(pos_after_start(&nums, 2, 2), Some(4));
    assert_eq!(pos_after_start(&nums, 3, 2), Some(4));
    assert_eq!(pos_after_start(&nums, 4, 2), None);
}

#[test]
fn test_shortest_way() {
    let src = "abca".to_string();
    let tgt = "abcbca".to_string();
    assert_eq!(shortest_way(src, tgt), 2);
}