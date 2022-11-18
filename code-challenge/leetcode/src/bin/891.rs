
fn main() {
    println!("sub seq width!");
}

fn sum_subseq_widths(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0
    }

    let mut nums = nums.clone();
    let mut ret = 0;

    nums.sort();

    for i in 0..nums.len()-1 {
        for j in i+1..nums.len() {
            ret += (nums[j] - nums[i])*(1<<(j-i-1))
        }
    }

    return ret 
}

fn dummy_sum(nums: Vec<i32>) -> i32 {
    return 0
}

#[test]
fn test_sum_subseq_widths() {
    let nums = vec![2, 2, 1];
    assert_eq!(sum_subseq_widths(nums), 3);
}