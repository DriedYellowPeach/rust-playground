use std::{
    cmp::Ordering,
    ops::{Add, Sub}, fmt::Debug,
};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let (mut left, mut right) = (0, nums.len() - 1);

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
pub fn three_sum<T>(nums: Vec<T>, target: T) -> Vec<Vec<T>>
where
    T: PartialOrd + Ord + Add<Output = T> + Sub<Output = T> + Copy,
{
    let mut i = 0;
    let mut out = vec![];
    let mut nums = nums;
    nums.sort();

    // loop until i == nums.len
    while i < nums.len() {
        if nums[i] > target {
            break;
        }
        let aim = target - nums[i];
        let mut left = i + 1;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] > aim {
                break;
            }

            match (nums[left] + nums[right]).cmp(&aim) {
                Ordering::Less => left += 1,
                Ordering::Greater => right -= 1,
                Ordering::Equal => {
                    out.push(vec![nums[i], nums[left], nums[right]]);
                    left += 1;
                    right -= 1;
                    // forward left until it is different from left_history
                    while left < right && nums[left] == nums[left - 1] {
                        left += 1;
                    }
                }
            }
        }
        i += 1;
        while i < nums.len() && nums[i - 1] == nums[i] {
            i += 1;
        }
    }
    out
}

#[test]
fn test_three_sum() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let target = 0;
    let out = three_sum(nums, target);
    assert_eq!(out, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
}

// return all indexes, four sum T18
pub fn four_sum<T>(nums: Vec<T>, target: T) -> Vec<Vec<T>>
where
    T: PartialOrd + Ord + Add<Output = T> + Sub<Output = T> + Copy,
{
    let mut i = 0;
    let mut nums = nums;
    nums.sort();
    let mut out = vec![];

    while i < nums.len() {
        let mut j = i + 1;
        while j < nums.len() {
            let aim = target - nums[i] - nums[j];
            let mut left = j + 1;
            let mut right = nums.len() - 1;

            while left < right {
                match (nums[left] + nums[right]).cmp(&aim) {
                    Ordering::Less => left += 1,
                    Ordering::Greater => right -= 1,
                    Ordering::Equal => {
                        out.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                        while left < right && nums[left - 1] == nums[left] {
                            left += 1;
                        }
                    }
                }
            }
            j += 1;
            while j < nums.len() && nums[j - 1] == nums[j] {
                j += 1;
            }
        }
        i += 1;
        while i < nums.len() && nums[i - 1] == nums[i] {
            i += 1;
        }
    }
    out
}

#[test]
fn test_four_sum() {
    // let nums = vec![1, 0, -1, 0, -2, 2];
    // let target = 0;
    // let out = four_sum(nums, target);
    // assert_eq!(out, vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]);
    let nums = vec![1, -2, -5, -4, -3, 3, 3, 5];
    let target = -11;
    let out = four_sum(nums, target);
    assert_eq!(out, vec![vec![-5, -4, -3, 1]]);
}

pub fn merge_display<T: PartialOrd + Ord + Debug>(nums1: Vec<T>, nums2: Vec<T>) {
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            println!("{:?}", nums1[i]);
            i += 1;
        } else {
            println!("{:?}", nums2[j]);
            j += 1;
        }
    }

    while i < nums1.len() {
        println!("{:?}", nums1[i]);
        i += 1;
    }

    while j < nums2.len() {
        println!("{:?}", nums2[j]);
        j += 1;
    }
}

#[test]
fn test_merge_display() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 3, 4, 5];
    merge_display(nums1, nums2);
}

fn merge_sort<T: PartialOrd + Ord + Default + Copy>(nums: &mut [T], l: usize, r: usize)  {
    if l < r {
        let m = l + (r - l) / 2;
        merge_sort(nums, l, m);
        merge_sort(nums, m + 1, r);
        merge(nums, l, m, r);
    }
}

#[test]
fn test_merge_sort() {
    let mut nums = vec![5, 4, 3, 2, 1];
    let length = nums.len();
    merge_sort(&mut nums, 0, length - 1);
    assert_eq!(nums, vec![1, 2, 3, 4, 5]);
}


fn merge<T: PartialOrd + Ord + Default + Copy>(nums: &mut [T], l: usize, m: usize, r: usize) {
    let left_len = m - l + 1;
    let right_len = r - m;

    let mut left = Vec::new();
    left.resize_with(left_len, T::default);
    let mut right = Vec::new();
    right.resize_with(right_len, T::default);

    for i in l..=r {
        if i <= m {
            left[i - l] = nums[i];
        } else {
            right[i - m - 1] = nums[i];
        }
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < left_len && j < right_len {
        if left[i] <= right[j] {
            nums[k] = left[i];
            i += 1;
        } else {
            nums[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left_len {
        nums[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right_len {
        nums[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[test]
fn test_merge() {
    let mut nums = vec![4, 5, 6, 7, 8, 1, 2, 3, 9, 10];
    merge(&mut nums, 0, 4, 9);
    assert_eq!(nums, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

