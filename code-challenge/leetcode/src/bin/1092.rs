fn main() {
    unimplemented!();
}
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Left,
    UpLeft,
    Empty,
}

fn display_dp(dp: &[Vec<(usize, Direction)>]) {
    for i in 0..dp.len() {
        for j in 0..dp[0].len() {
            match dp[i][j].1 {
                Direction::Up => print!("↑ "),
                Direction::Left => print!("← "),
                Direction::UpLeft => print!("↖ "),
                Direction::Empty => print!("o "),
            }
        }
        println!();
    }
}

fn display_sequence(s: &[u8], t: &[u8], dp: &[Vec<(usize, Direction)>]) -> String {
    let mut i = (dp.len() - 1) as isize;
    let mut j = (dp[0].len() - 1) as isize;
    let mut out = Vec::new();

    while i >= 0 && j >= 0 {
        let (_l, d) = dp[i as usize][j as usize];
        match d {
            Direction::Up => {
                out.push(t[i as usize] as char);
                i -= 1;
            }
            Direction::Left => {
                out.push(s[j as usize] as char);
                j -= 1;
            }
            Direction::UpLeft => {
                out.push(s[j as usize] as char);
                i -= 1;
                j -= 1;
            }
            Direction::Empty => {
                break;
            }
        }
    }

    while i >= 0 {
        out.push(t[i as usize] as char);
        i -= 1;
    }

    while j >= 0 {
        out.push(s[j as usize] as char);
        j -= 1;
    }

    out.iter().rev().collect::<String>()
}

pub fn longest_common_sequence(s: &str, t: &str) -> String {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut dp = vec![vec![(0, Direction::Empty); s.len()]; t.len()];

    // 1. init dp
    dp[0][0] = if t[0] == s[0] { (1, Direction::UpLeft) } else { (0, Direction::Empty) };

    for j in 1..s.len() {
        if t[0] == s[j] {
            dp[0][j] = (1, Direction::UpLeft);
            continue;
        }

        if dp[0][j - 1].0 == 1 {
            dp[0][j] = (1, Direction::Left);
        }
    }

    for i in 1..t.len() {
        if t[i] == s[0] {
            dp[i][0] = (1, Direction::UpLeft);
            continue;
        }

        if dp[i - 1][0].0 == 1 {
            dp[i][0] = (1, Direction::Up);
        }
    }

    // 2. fill dp
    for ti in 1..t.len() {
        for sj in 1..s.len() {
            if t[ti] == s[sj] {
                dp[ti][sj] = (dp[ti - 1][sj - 1].0 + 1, Direction::UpLeft);
                continue;
            }

            // t[ti] != s[sj], which means we can't extend our sequence
            // so we try our best from the top and left
            if dp[ti - 1][sj].0 > dp[ti][sj - 1].0 {
                dp[ti][sj] = (dp[ti - 1][sj].0, Direction::Up);
            } else {
                dp[ti][sj] = (dp[ti][sj - 1].0, Direction::Left);
            }
        }
    }

    // 3. display
    // display_dp(&dp);
    display_sequence(s, t, &dp)
}

#[allow(dead_code)]
fn shortest_comman_super_sequence(str1: String, str2: String) -> String {
    longest_common_sequence(&str1, &str2)
}

#[test]
fn test() {
    let s = shortest_comman_super_sequence("aaa".to_string(), "aaa".to_string());
    println!("{s}");
    let s = shortest_comman_super_sequence("dabac".to_string(), "cabc".to_string());
    println!("{s}");
    let s = shortest_comman_super_sequence("dabac".to_string(), "cabcf".to_string());
    println!("{s}");
    let s = shortest_comman_super_sequence("acb".to_string(), "bbb".to_string());
    println!("{s}");
    let s = shortest_comman_super_sequence("acbbcccaa".to_string(), "bbbcaaaaa".to_string());
    println!("{s}");
}
