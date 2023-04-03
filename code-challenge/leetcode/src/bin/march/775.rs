fn main() {

}

use std::cmp;

fn is_deal_permutation(nums: Vec<i32>) -> bool {
    if nums.len() <= 2 {
        return true
    }

    // set initial state: max_a, max_b
    let mut max_a = nums[0];
    let mut max_b = cmp::max(nums[0], nums[1]);
    // let mut max_b = if nums[1] > nums[0] { nums[1] } else { nums[0] };
    // let max_b = nums[..2].iter().max();
    // let mut max_b = match nums[..2].iter().max() {
    //     Some(n) => { *n }, 
    //     None => { return true }
    // };

    for v in nums.iter().skip(2) {
        if *v < max_a {
            return false
        }
        max_a = max_b;
        // max_b = if *v > max_b { *v } else { max_b };
        max_b = cmp::max(*v, max_b);
    }

    return true
}

#[test]
fn test_is_ideal_permutation() {
    assert_eq!(is_deal_permutation(vec![0, 1, 2]), true);
    assert_eq!(is_deal_permutation(vec![0, 2, 1]), true);
    assert_eq!(is_deal_permutation(vec![1, 2, 0]), false);

    assert_eq!(is_deal_permutation(vec![1, 2, 0]), false);
}