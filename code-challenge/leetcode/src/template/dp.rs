// Longest Common Subsequence
mod lcs {
    #[derive(Clone, Copy, Debug)]
    enum Direction {
        Up,
        Left,
        UpLeft,
        Empty,
    }

    fn display_sequence(s: &[u8], dp: &[Vec<(usize, Direction)>]) {
        let mut i = (dp.len() - 1) as isize;
        let mut j = (dp[0].len() - 1) as isize;
        let mut out = Vec::new();
        // println!("{}", String::from_utf8_lossy(s));

        while i >= 0 && j >= 0 {
            let (_l, d) = dp[i as usize][j as usize];
            match d {
                Direction::Up => {
                    i -= 1;
                }
                Direction::Left => {
                    j -= 1;
                }
                Direction::UpLeft => {
                    // println!("{:?}", s[j as usize] as char);
                    out.push(s[j as usize] as char);
                    i -= 1;
                    j -= 1;
                }
                Direction::Empty => {
                    // println!("{:?}", s[j as usize] as char);
                    out.push(s[j as usize] as char);
                    break;
                }
            }
        }
        println!("{:?}", out.iter().rev().collect::<String>());
    }

    pub fn longest_common_sequence(s: &str, t: &str) -> usize {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut dp = vec![vec![(0, Direction::Empty); s.len()]; t.len()];

        // 1. init dp
        dp[0][0] = (if t[0] == s[0] { 1 } else { 0 }, Direction::Empty);

        for j in 1..s.len() {
            if t[0] == s[j] {
                dp[0][j] = (1, Direction::Empty);
                continue
            }

            if dp[0][j - 1].0 == 1 {
                dp[0][j] = (1, Direction::Left);
            }
        }

        for i in 1..t.len() {
            if t[i] == s[0] {
                dp[i][0] = (1, Direction::Empty);
                continue
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
        display_sequence(s, &dp);

        dp[t.len() - 1][s.len() - 1].0
    }

    #[test]
    fn test_longest_common_sequence() {
        let s = "abcde";
        let t = "ace";
        let out = longest_common_sequence(s, t);
        assert_eq!(out, 3);

        let s = "abkkcdzze";
        let t = "akczef";
        let out = longest_common_sequence(s, t);
        assert_eq!(out, 5);
    }
}
