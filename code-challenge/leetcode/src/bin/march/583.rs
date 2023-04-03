// type: string, dp, subsequence
// solution: dp(i) = g(dp(0), dp(1), ... , dp(i-1))

fn main() {
    println!("give string a and b, find the longest common sub sequence!");
}

use std::collections::HashMap;

// fn min_distance(word1: String, word2: String) -> i32 {
//     let s_long;
//     let s_short;

//     if word1.len() > word2.len() {
//         s_long = word1.as_bytes();
//         s_short = word2.as_bytes();
//     } else {
//         s_long = word2.as_bytes();
//         s_short = word1.as_bytes();
//     }

//     let skip_and_pos = |jmp: usize, target: usize| -> (Option<usize>, usize) {
//         let pos = s_long.iter().skip(jmp).position(|&b| b == s_short[target]);
//         match pos  {
//             Some(idx) => {
//                 (Some(idx+jmp), 1)
//             },
//             None => {
//                 (None, 0)
//             }
//         }
//     };

//     let mut dp: Vec<HashMap<Option<usize>, usize>> = vec![HashMap::new(); s_short.len()];
//     let pair = skip_and_pos(0, 0);
//     dp[0].insert(pair.0, pair.1);

//     for i in 1..s_short.len() {
//         for j in 0..i {
//             let (front, back) = dp.split_at_mut(i);
//             let (prev, cur) = (&front[j], &mut back[0]);

//             // insert not greedy solution, find from 0 and length is 1 or 0
//             let pair = skip_and_pos(0, i);
//             cur.insert(pair.0, pair.1);
//             // cur.insert(s_long.iter().position(|&b| b == s_short[i]), 1);
//             for (&idx, &length) in prev {
//                 match idx {
//                     Some(n) => {
//                         // println!("{:?} {:?}", n, i);
//                         let pair = skip_and_pos(n+1, i);
//                         let old_len = cur.get(&pair.0).copied().unwrap_or_default();
//                         // let old_len = match old_len {
//                         //     Some(&l) => { l },
//                         //     None => { 0 }
//                         // };
//                         // println!("{:?} {:?} {:?}", pair.0, pair.1, old_len);
//                         cur.insert(pair.0, std::cmp::max(pair.1 + length, old_len));
//                     }
//                     None => {
//                         let pair = skip_and_pos(0, i);
//                         // let old_len = cur.get(&pair)
//                         let old_len = cur.get(&pair.0).copied().unwrap_or_default();
//                         // let old_len = match old_len {
//                         //     Some(&l) => { l },
//                         //     None => { 0 }
//                         // };
//                         cur.insert(pair.0, std::cmp::max(pair.1, old_len));
//                     }
//                 }
//             }
//         }
//     }

//     println!("{:?}", dp);
//     return *dp.last().unwrap().values().max().unwrap() as i32;
// }

fn min_distance(word1: String, word2: String) -> i32 {
    let s_long;
    let s_short;

    if word1.len() > word2.len() {
        s_long = word1.as_bytes();
        s_short = word2.as_bytes();
    } else {
        s_long = word2.as_bytes();
        s_short = word1.as_bytes();
    }

    let skip_and_pos = |jmp: usize, target: usize| -> (usize, usize) {
        let pos = s_long.iter().skip(jmp).position(|&b| b == s_short[target]);
        match pos  {
            Some(idx) => {
                (idx+jmp+1, 1)
            },
            None => {
                (0, 0)
            }
        }
    };

    let mut dp: Vec<HashMap<usize, usize>> = vec![HashMap::new(); s_short.len()];
    let pair = skip_and_pos(0, 0);
    dp[0].insert(pair.0, pair.1);

    for i in 1..s_short.len() {
        for j in 0..i {
            let (front, back) = dp.split_at_mut(i);
            let (prev, cur) = (&front[j], &mut back[0]);

            // insert not greedy solution, find from 0 and length is 1 or 0
            let pair = skip_and_pos(0, i);
            cur.insert(pair.0, pair.1);
            // cur.insert(s_long.iter().position(|&b| b == s_short[i]), 1);
            for (&idx, &length) in prev {
                let pair = skip_and_pos(idx, i);
                let new_pair = (pair.0, pair.1+length);
                let old_len = cur.get(&pair.0).copied().unwrap_or_default();
                if old_len < new_pair.1 {
                    cur.insert(new_pair.0, new_pair.1);
                }
            }
        }
    }

    println!("{:?}", dp);
    let l = *dp.last().unwrap().values().max().unwrap();
    return (s_long.len() + s_short.len() - 2*l) as i32
}

#[test]
fn test_min_distance() {
    let word1 = "sea".to_string();
    let word2 = "tea".to_string();

    assert_eq!(min_distance(word1, word2), 2);

    let word1 = "hello".to_string();
    let word2 = "elloh".to_string();

    assert_eq!(min_distance(word1, word2), 2);

    let word1 = "helloxxxyyy".to_string();
    let word2 = "oellxxxyyy".to_string();

    assert_eq!(min_distance(word1, word2), 3);
}

#[test]
fn test_skip_and_pos() {
    let s_short = "sea".as_bytes();
    let s_long = "tea".as_bytes();
    let skip_and_pos = |jmp: usize, target: usize| -> (Option<usize>, usize) {
        let pos = s_long.iter().skip(jmp).inspect(|&b| {println!("{}", b); }).position(|&b| b == s_short[target]);
        match pos  {
            Some(idx) => {
                (Some(idx+jmp), 1)
            },
            None => {
                (None, 0)
            }
        }
    };

    assert_eq!(skip_and_pos(2, 2), (Some(2), 1));
}