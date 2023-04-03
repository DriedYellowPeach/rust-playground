fn main() {
    unimplemented!();
}

use std::collections::VecDeque;
fn moving_count(m: i32, n: i32, k: i32) -> i32 {
    // directions: up down left right
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let point_in_range = |ref row, ref col| (0..m).contains(row) && (0..n).contains(col);
    let bit_sum = |mut x: i32| -> i32 {
        let mut sum = 0;
        while x != 0 {
            sum += x % 10;
            x /= 10;
        }
        sum
    };
    let point_valid = |row, col| -> bool { bit_sum(row) + bit_sum(col) <= k };

    let mut seen = vec![vec![false; n as usize]; m as usize];
    let mut queue = VecDeque::<(i32, i32)>::new();
    seen[0][0] = true;
    queue.push_back((0, 0));

    while !queue.is_empty() {
        let pt = queue.pop_front().unwrap();
        for dir in directions {
            let next = (pt.0 + dir.0, pt.1 + dir.1);
            let row = next.0;
            let col = next.1;
            if point_in_range(row, col)
                && !seen[row as usize][col as usize]
                && point_valid(row, col)
            {
                seen[row as usize][col as usize] = true;
                queue.push_back(next);
            }
        }
    }

    seen.into_iter().flatten().filter(|x| *x).count() as i32
}

#[test]
fn test_moving_count() {
    let m = 2;
    let n = 3;
    let k = 1;
    assert_eq!(moving_count(m, n, k), 3);
    let m = 3;
    let n = 1;
    let k = 0;
    assert_eq!(moving_count(m, n, k), 1);
}

#[test]
fn test_bit_sum() {
    let bit_sum = |mut x: usize| -> usize {
        let mut sum = 0;
        while x != 0 {
            sum += x % 10;
            x /= 10;
        }
        sum
    };
    assert_eq!(bit_sum(23), 5);
    assert_eq!(bit_sum(35), 8);
    assert_eq!(bit_sum(5), 5);
}

#[test]
fn test_flattern() {
    let seen = [
        [true, false, false],
        [true, false, false],
        [true, false, false],
    ];
    assert_eq!(seen.into_iter().flatten().filter(|x| *x).count(), 3);
}
