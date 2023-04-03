
fn main() {
    unimplemented!();
}

use leetcode::template::dp::lcs;

#[allow(dead_code)]
fn cal_lcs(text1: String, text2: String) -> i32 {
    lcs::longest_common_sequence(&text1, &text2) as i32
}
