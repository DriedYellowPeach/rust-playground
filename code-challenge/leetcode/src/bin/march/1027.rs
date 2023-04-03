
fn main() {
    println!("find the longest arithmetic sequence");
}

use std::collections::HashMap;

fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut dp:Vec<HashMap<i32, i32>> = vec![HashMap::new();nums.len()];

    for mp in &mut dp {
        mp.insert(-1, 1);
    }

    for i in 1..nums.len() {
        for j in 0..i {
            // get [i] - [j]
            let ij_diff = nums[i] - nums[j];
            // get [i] [j] map 
            let (front, back) = dp.split_at_mut(i);
            let front_map = &mut front[j];
            let back_map = &mut back[0];
            // j as first, i as second
            back_map.insert(ij_diff, 2);

            // j as second or larger
            if let Some(&length) = front_map.get(&ij_diff) {
                ret = std::cmp::max(ret, length+1);
                back_map.insert(ij_diff, length+1);
            }

            // for (&dif, &mut length) in front_map {
            //     if dif == -1 {
            //         back_map.insert(nums[i] - nums[j], length + 1);
            //         continue
            //     }

            //     if dif == nums[i] - nums[j] {

            //     }

            // }
        }
    }

    println!("{:?}", dp);

    return ret;
}

#[test]
fn test_longest_arith_seq_length() {
    let v = vec![1, 2, 3, 7, 4];
    assert_eq!(longest_arith_seq_length(v), 4);
}