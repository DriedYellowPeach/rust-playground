fn main() {}

fn count_triplets(nums: Vec<i32>) -> i32 {
    let mut table = Vec::<Vec<usize>>::with_capacity(32);
    for i in 0..32 {
        let mut ith_bit_zero = Vec::new();
        let test_bit = 0x1 << i;
        for (i, &n) in nums.iter().enumerate() {
            if n & test_bit == 0 {
                ith_bit_zero.push(i);
            }
        }

        table.push(ith_bit_zero);
    }

    0
}

use std::collections::HashMap;
fn count_triplets_v2(nums: Vec<i32>) -> i32 {
    let mut bit_and = HashMap::new();
    let mut ret = 0;

    for &n in &nums {
        for &m in &nums {
            *bit_and.entry(m & n).or_insert(0) += 1;
        }
    }

    for &n in &nums {
        for (&k, &v) in &bit_and {
            if n & k == 0 {
                ret += v;
            }
        }
    }

    ret
}

#[test]
fn test_count_triplets() {
    let nums = vec![2, 1, 3];
    assert_eq!(count_triplets_v2(nums), 12);
}

#[test]
fn tessdft_ith_bit_zero() {
    let nums = vec![2, 1, 3];
    let mut table = Vec::<Vec<usize>>::with_capacity(32);
    for i in 0..32 {
        let mut ith_bit_zero = Vec::new();
        let test_bit = 0x1 << i;
        for (i, &n) in nums.iter().enumerate() {
            if n & test_bit == 0 {
                ith_bit_zero.push(i);
            }
        }

        table.push(ith_bit_zero);
    }

    println!("{table:?}");
}
