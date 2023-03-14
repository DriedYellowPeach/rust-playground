fn main() {
    unimplemented!();
}

use std::collections::HashMap;
fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
    let rem = nums.iter().fold(0, |mut acc, x| {
        acc = (acc + x) % p;
        acc
    });
    if rem == 0 {
        return 0;
    }

    let mut min_out = nums.len();
    let mut pre_rems = HashMap::<i32, usize>::new();

    let mut pre_rem = 0;


    // sub[i, j)
    for j in 0..=nums.len() {
        // let pre_rem = presum[j] % p;
        pre_rem = if j == 0 { 0 } else { (pre_rem % p + nums[j-1] % p) % p };
        *pre_rems.entry(pre_rem).or_insert(j) = j;
        let target_rem = (pre_rem - rem + p) % p;
        // println!("j: {j} target_rem: {target_rem}");
        if let Some(i) = pre_rems.get(&target_rem) {
            println!("{i} {j} {min_out}");
            min_out = std::cmp::min(min_out, j - i);
        }
    }

    if min_out == nums.len() {
        -1
    } else {
        min_out as i32
    }
}

#[test]
fn test_min_subarray() {
    let nums = vec![4, 5, 7];
    let p = 3;
    assert_eq!(min_subarray(nums, p), 1);

    let nums = vec![1, 4, 5, 7, 4];
    let p = 5;
    assert_eq!(min_subarray(nums, p), 1);

    let nums = vec![1, 4, 5, 6, 4];
    let p = 7;
    assert_eq!(min_subarray(nums, p), 1);

    let nums = vec![8,32,31,18,34,20,21,13,1,27,23,22,11,15,30,4,2];
    let p = 148;
    assert_eq!(min_subarray(nums, p), 7);
}

#[test]
fn test_presum() {
    let mut acc = 0;
    let nums = [1, 2, 3, 4, 5];
    let presum = (0..1)
        .chain(nums.iter().map(|n| {
            acc += *n;
            acc
        }))
        .collect::<Vec<i32>>();
    println!("{presum:?}");
}

#[test]
fn test_neg_mod() {
    assert_eq!(-5 % 3, -2);
}

#[test]
fn test_cal_mod_without_overflow() {
    let a = [1000000000, 1000000000, 1000000000];
    // assert_eq!(a.iter().sum::<i32>() % 3, 0);
    let rem = a.iter().fold(0, |mut acc, x| {
        acc = (acc % 3 + x % 3) % 3;
        acc
    });
    assert_eq!(rem, 0);

    let nums = vec![8,32,31,18,34,20,21,13,1,27,23,22,11,15,30,4,2];
    let p = 148;
    assert_eq!(nums.iter().fold(0, |mut acc, x| {
        acc = (acc % p + x % p) % p;
        acc
    }), nums.iter().sum::<i32>() % p)
}
