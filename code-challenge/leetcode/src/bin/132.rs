// type: dp, palindrome
//
// solution: 
// get tbl and use dp(i) = g(dp(0), dp(1), ... , dp(i))

fn main() {
    println!("find the min cut solution of palindrome partition!");
}

fn get_partition_tbl(s: &str) -> Vec<Vec<bool>> {
    let s = s.as_bytes();
    let length = s.len();
    let mut tbl: Vec<Vec<bool>> = vec![vec![true; length]; length];

    for i in (0..length-1).rev() {
        for j in (i+1..length) {
            tbl[i][j] = (s[i] == s[j]) && tbl[i+1][j-1];
        }
    }

    return tbl
}

fn min_cut(s: String) -> i32 {
    let tbl = get_partition_tbl(&s);
    let s = s.as_bytes();
    let length = s.len();  
    let mut dp: Vec<i32> = vec![length as i32; length];
    // let mut small = length as i32;
    dp[0] = 1;

    for i in 1..length {
        if tbl[0][i] {
            dp[i] = 1;
        }

        for j in 0..i {
            if tbl[j+1][i] {
                dp[i] = std::cmp::min(dp[i], dp[j]+1);
                // small = std::cmp::min(small, dp[j] + 1);
            }
        } 
    }
    // println!("{:?}", dp);

    return *dp.last().unwrap()
}

#[test]
fn test_min_cut() {
    assert_eq!(min_cut("abab".to_string()), 2);
    assert_eq!(min_cut("ababcbcb".to_string()), 2);
}