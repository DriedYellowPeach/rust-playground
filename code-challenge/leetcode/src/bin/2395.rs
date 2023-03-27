
fn main() {
    unimplemented!();
}

use std::collections::HashSet;

#[allow(dead_code)]
fn find_subarrays(nums: Vec<i32>) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let (mut left, mut right) = (0, 1);
    let mut seen = HashSet::new();

    while right < nums.len() {
        // sum
        let sum: isize = nums[left] as isize + nums[right] as isize;
        // and then or else
        match seen.get(&sum) {
            Some(_) => return true,
            None => seen.insert(sum),
        };
        // move windown
        left += 1;
        right += 1;
    }

    false
}

#[test]
fn test_find_subarrays() {
    assert!(find_subarrays(vec![4, 2, 4]));
    assert!(!find_subarrays(vec![1, 2, 3, 4, 5]));
}
