use std::collections::HashMap;


fn main() {
    println!("find the fibonacci subsequence with longest length!");
}

use std::cmp;
fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut dp: Vec<HashMap<i32, i32>> = vec![HashMap::new();arr.len()];

    for m in &mut dp {
        m.insert(-1, 1);
    }

    for i in 1..arr.len() {
        for j in 0..i {
            let (front, back) = dp.split_at_mut(i);
            let b_mp = &mut front[j];
            let c_mp = &mut back[0];
            // j as first
            let v = b_mp.get(&-1).unwrap();
            c_mp.insert(arr[j], v+1);

            // j as second or other...
            if let Some(&length) = b_mp.get(&(arr[i] - arr[j])) {
                ret = cmp::max(ret, length+1);
                c_mp.insert(arr[j], length+1);
            }
            // for (&k, &mut v) in b_mp {
            //     if k == -1 {
            //         // check if bigger, should insert?
            //         c_mp.insert(arr[j], v+1);
            //         continue;
            //     } 

            //     if k + arr[j] == arr[i] {
            //         if let Some(&l) = c_mp.get(&k) {
            //             let j_len = cmp::max(l, v+1);
            //             // if arr[i] == 5 { println!("{}, {}", j_len, ret)};
            //             ret = cmp::max(ret, j_len);
            //             c_mp.insert(arr[j], j_len);
            //         }
            //     }
            // }
        }
    }

    // println!("{:?}", dp);
    return ret;
}

#[test]
fn test_len_longest_fib_sub_seq() {
    let v = vec![1, 2, 3, 5];
    assert_eq!(len_longest_fib_subseq(v), 4);
}