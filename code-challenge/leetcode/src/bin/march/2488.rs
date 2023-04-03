use std::collections::HashMap;

fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn count_subarray(nums: Vec<i32>, k: i32) -> i32 {
    let k_idx = nums.iter().position(|&val| val == k).expect("invalid input, no {k} in nums");
    let (mut left, mut right) = (k_idx as i32 - 1, k_idx as i32 + 1);
    let (mut left_gap, mut right_gap) = (0, 0);
    let mut gaps = vec![0; nums.len()];
    let mut gap_map = HashMap::<i32, i32>::new();

    // get the gap array
    while left >= 0 || (right as usize) < nums.len() {
        if left >= 0 {
            left_gap += if nums[left as usize] < k { 1 } else { -1 };
            gaps[left as usize] = left_gap;
            left -= 1;
            *gap_map.entry(left_gap).or_insert(0) += 1;
        }

        if (right as usize) < nums.len() {
            right_gap += if nums[right as usize] < k { 1 } else { -1 };
            gaps[right as usize] = right_gap;
            right += 1;
        }
    }

    let target_gap = (0, -1);
    // already have sub array [k]
    let mut out = 0;


    for i in k_idx..nums.len() {
        let left_gap1 = target_gap.0 - gaps[i];
        let left_gap2 = target_gap.1 - gaps[i];

        if gaps[i] == target_gap.0 || gaps[i] == target_gap.1 {
            out += 1;
        }

        // better solution here
        out += gap_map.get(&left_gap1).cloned().unwrap_or(0);
        out += gap_map.get(&left_gap2).cloned().unwrap_or(0);
    }

    // println!("{gaps:?}");
    // println!("{gap_map:?}");
    out
}

#[test]
fn test_count_subarray() {
    let nums = vec![3, 2, 1, 4, 5];
    let k = 4;
    assert_eq!(count_subarray(nums, k), 3);
}
