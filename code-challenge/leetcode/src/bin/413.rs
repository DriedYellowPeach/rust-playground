
fn main() {
    println!("find the number of all arithmetic sub slice!");
}

fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    // dp[i] is (diff, nums)
    let mut dp:Vec<Vec<(i32, i32)>> = vec![Vec::new(); nums.len()];
    let mut ret = 0;

    for i in 1..dp.len() {
        let (front, back) = dp.split_at_mut(i);
        let (prev, cur) = (front.last_mut().unwrap(), back.first_mut().unwrap());
        let diff = nums[i] - nums[i-1];
        let mut has_diff = false;
        // push (nums[i] - nums[i-1], 0)

        for pair in prev {
            if diff == pair.0 {
                has_diff = true;
                cur.push((diff, pair.1+1));
                ret += pair.1 + 1;
            }
        }

        // add new diff, means a posibility to have new slice
        if !has_diff {
            cur.push((diff, 0));
        }
    }

    println!("{:?}", dp);

    return ret;
}

#[test]
fn test_number_of_arithmetic_slices() {
    let nums = vec![1, 2, 3, 4, 6, 8, 10];
    assert_eq!(number_of_arithmetic_slices(nums), 6);
}