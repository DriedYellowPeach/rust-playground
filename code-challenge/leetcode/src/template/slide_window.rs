
#[allow(dead_code)]
fn sliding_window<T>(nums: &[T], winsz: usize) {
    let mut l = 0;
    // slide window
    for r in winsz..nums.len() {
        // update window
        while r - l + 1 > winsz {
            // delete nums[l]
            l += 1;
        }
        // update ans
    }
}

use std::ops::{Add, AddAssign, SubAssign};

#[allow(dead_code)]
fn max_window<T>(nums: &[T], winsize: usize) -> T
where T: Add<Output = T> + PartialOrd + Ord + AddAssign + Default + SubAssign + Copy
{
    let mut l = 0;
    let mut r = 0;
    let mut sum = T::default();
    
    while r < winsize {
        // update window
        sum += nums[r];
        r += 1;
    }

    let mut out = sum;

    while r < nums.len() {
        // update ans
        sum += nums[r];
        sum -= nums[l];
        out = std::cmp::max(out, sum);
        l += 1;
        r += 1;
    }

    out
}

#[test]
fn test_max_window() {
    let nums = [10, -2, 3, 4, 5, -6, 7, 8, -9];
    assert_eq!(max_window(&nums, 3), 12);
}

