fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
    let mut output = 0;

    for i in &nums {
        if nums.binary_search(&(i + diff)).is_ok() && nums.binary_search(&(i + diff * 2)).is_ok() {
            output += 1;
        }
    }

    output
}

#[test]
fn test_arithmetic_triplets() {
    assert_eq!(arithmetic_triplets(vec![2, 4, 6, 8, 10], 2), 3);
}
