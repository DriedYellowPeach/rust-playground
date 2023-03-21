use std::{cmp::Ordering, ops::Add};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let (mut left, mut right) =  (0, nums.len() - 1);

    while left < right {
        let sum = nums[left] + nums[right];
        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => return vec![left, right],
        }
    }

    vec![]
}

// return all indexes, three sum T15
pub fn three_sum<T>(nums: Vec<T>, target: T) -> Vec<Vec<usize>> 
where T: PartialOrd + Add
{
    vec![]
}

// return all indexes, four sum T18
pub fn four_sum<T>(nums: Vec<T>, target: T) -> Vec<Vec<usize>> 
where T: PartialOrd + Add
{
    vec![]
}
