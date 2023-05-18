fn merget_stones(stones: Vec<i32>, k: i32) -> i32 {
    if (stones.len() as i32 - k) % (k - 1) != 0 {
        return -1;
    }

    let mut stones = stones;
    let mut out = 0;

    while stones.len() > 1 {
        let mut i = 0;
        let mut j = 0;
        let mut sum = 0;
        while j < k {
            sum += stones[j as usize];
            j += 1;
        }
        let mut sum_min = sum;
        let mut min_idx = 0;

        while j < stones.len() as i32 {
            sum += stones[j as usize];
            sum -= stones[i as usize];
            i += 1;
            j += 1;

            if sum < sum_min {
                sum_min = sum;
                min_idx = i;
            }
        }

        out += sum_min;
        // println!("{} {}", min_idx, sum_min);

        stones.drain(min_idx as usize + 1..min_idx as usize + k as usize);
        stones[min_idx as usize] = sum_min;
        // println!("{:?}", stones);
    }

    out
}

#[test]
fn test_merge_stones() {
    assert_eq!(merget_stones(vec![3, 2, 4, 1], 2), 20);
    assert_eq!(merget_stones(vec![3, 2, 4, 1], 3), -1);
    assert_eq!(merget_stones(vec![3, 5, 1, 2, 6], 3), 25);
}
