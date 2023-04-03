
fn main() {
    println!("get the longest subset in which every pair can be diveded!");
}

fn largest_divisiible_subset(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums.clone();
    // dp[i] (last item,length)
    // let dp: Vec<(i32, i32)> = vec![1; nums.len()];
    let mut dp: Vec<(i32, i32)> = Vec::with_capacity(nums.len());
    let mut ret: Vec<i32> = Vec::new();

    nums.sort();
    for i in 0..nums.len() {
        dp.push((-1, 1));
    }

    for i in 1..nums.len() {
        let l = dp[i];
        for j in 0..i {
            if nums[i] % nums[j] == 0 {
                if dp[j].1 + 1 > l.1 {
                    dp[i] =(j as i32, dp[j].1+1);
                }
            }
        }
    }
    
    let max_length = dp.iter().enumerate().max_by(|x, y| x.1.cmp(&y.1)).unwrap();
    let max_length = max_length.0;
    ret.push(nums[max_length]);
    let mut item = &dp[max_length];

    while item.0 != -1 {
        ret.push(nums[item.0 as usize]);
        item = &dp[item.0 as usize];
    }
    return ret
}

#[test]
fn test_largest_divisible_subset() {
    let nums = vec![1, 2, 4, 8];
    assert_eq!(largest_divisiible_subset(nums), vec![8, 4, 2, 1]);
}