fn main() {
    unimplemented!();
}

fn answer_quries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();
    let mut presum = vec![0; nums.len()];
    presum[0] = nums[0];

    for i in 1..presum.len() {
        presum[i] = presum[i - 1] + nums[i];
    }

    let mut out = vec![0; queries.len()];
    for (i, o) in out.iter_mut().enumerate() {
        *o = match presum.binary_search(&queries[i]) {
            Ok(idx) => idx as i32 + 1,
            Err(idx) => idx as i32,
        };
    }

    out
}

#[test]
fn test_answer_quries() {
    let nums = vec![4, 5, 2, 1];
    let queries = vec![3, 10, 21];
    assert_eq!(answer_quries(nums, queries), [2, 3, 4]);
}
