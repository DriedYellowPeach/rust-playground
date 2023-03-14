
fn main() {
    unimplemented!();
}

fn minimum_records(blocks: String, k: i32) -> i32 {
    let mut w_cnt = 0;
    let k = k as usize;
    let blk_len = blocks.len();
    if blk_len < k {
        panic!("invalid input");
    }

    let presum = blocks.bytes().map(|ch| {
        if ch == b'W' {
            w_cnt += 1;
        }
        w_cnt
    }).collect::<Vec<i32>>();
    println!("{:?}", presum);

    (0..=blk_len-k).map(|it| {
        if it == 0 {
            presum[k-1]
        } else {
            presum[it+k-1] - presum[it-1]
        }
    }).inspect(|it| print!("{it} ")).min().unwrap()
}

#[test]
fn test_minimum_records() {
    let blocks = "WBBWWBBWBW".to_string();
    assert_eq!(minimum_records(blocks, 7), 3);
}

#[test]
fn test_presum() {
    let blocks = "WBBWWBBWBW".to_string();
    let mut blk_cnt = 0;
    let presum = blocks.bytes().map(|ch| {
        if ch == b'B' {
            blk_cnt += 1;
        }
        blk_cnt
    }).collect::<Vec<i32>>();
    println!("{:?}", presum);
}
