fn main() {}

fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut dp = vec![0; nums.len()];
    dp[0] = nums[0];

    for i in 1..nums.len() {
        dp[i] = if dp[i - 1] > 0 {
            nums[i] + dp[i - 1]
        } else {
            nums[i]
        };
    }

    dp.into_iter().max().unwrap()
}

fn max_sub_array_v2(nums: Vec<i32>) -> i32 {
    let mut prev = nums[0];
    let mut largest = prev;

    for i in 1..nums.len() {
        let next = if prev > 0 {
            prev + nums[i]
        } else {
            nums[i]
        };

        largest = std::cmp::max(next, largest);
        prev =next;
    }

    largest
}

#[test]
fn test_max_sub_array() {
    let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    assert_eq!(max_sub_array_v2(nums), 6);
}
