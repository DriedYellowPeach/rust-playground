#[allow(dead_code)]
fn num_tile_possibilities(tiles: String) -> i32 {
    0
}

#[test]
fn test_num_tile_poss() {
    assert_eq!(num_tile_possibilities("AAB".to_string()), 8);
    assert_eq!(num_tile_possibilities("AAABBC".to_string()), 188);
}
