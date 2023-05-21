use std::collections::HashMap;

#[allow(dead_code)]
fn num_tile_possibilities(tiles: String) -> i32 {
    // counts is a character counts
    // example: {A: 3, B: 2, C:1}
    // let mut counts = HashMap::<char, usize>::new();
    let mut counts = [0usize; 26];
    for c in tiles.chars() {
        // let v = counts.entry(c).or_insert(0);
        // *v += 1;
        counts[c as usize - 'A' as usize] += 1;
    }
    dfs(&mut counts) as i32 - 1
}

fn dfs(counts: &mut [usize]) -> usize {
    let mut sum = 1;
    if counts.iter().sum::<usize>() == 0 {
        return 1;
    }

    for i in 0..26 {
        if counts[i] > 0 {
            counts[i] -= 1;
            sum += dfs(counts);
            counts[i] += 1;
        }
    }

    sum
}

#[test]
fn test_num_tile_poss() {
    assert_eq!(num_tile_possibilities("AAB".to_string()), 8);
    assert_eq!(num_tile_possibilities("AAABBC".to_string()), 188);
}
