// Longest Common Subsequence
pub mod lcs {
    #[derive(Clone, Copy, Debug)]
    enum Direction {
        Up,
        Left,
        UpLeft,
        Empty,
    }

    fn display_longest_super_sequence(t: &[u8], s: &[u8], dp: &[Vec<(usize, Direction)>]) {
        let mut i = (dp.len() - 1) as isize;
        let mut j = (dp[0].len() - 1) as isize;
        let mut out = Vec::new();

        while i >= 0 && j >= 0 {
            match dp[i as usize][j as usize].1 {
                Direction::Up => {
                    out.push(t[i as usize] as char);
                    i -= 1;
                }
                Direction::Left => {
                    out.push(s[j as usize] as char);
                    j -= 1;
                }
                Direction::UpLeft => {
                    // t[i] == s[j], so push either is ok
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

        println!("{}", out.iter().rev().collect::<String>());
    }

    fn display_dp(t: &[u8], s: &[u8], dp: &[Vec<(usize, Direction)>]) {
        for i in 0..=dp.len() {
            for j in 0..=dp[0].len() {
                if i == 0 && j == 0 {
                    print!("x ");
                    continue
                }

                if i == 0 {
                    print!("{} ", s[j - 1] as char);
                    continue
                }

                if j == 0 {
                    print!("{} ", t[i - 1] as char);
                    continue
                }

                let i = i - 1;
                let j = j - 1;
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

    fn display_sequence(s: &[u8], dp: &[Vec<(usize, Direction)>]) {
        let mut i = (dp.len() - 1) as isize;
        let mut j = (dp[0].len() - 1) as isize;
        let mut out = Vec::new();

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
                    out.push(s[j as usize] as char);
                    i -= 1;
                    j -= 1;
                }
                Direction::Empty => {
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
        dp[0][0] = (if t[0] == s[0] { 1 } else { 0 }, Direction::UpLeft);

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
        display_sequence(s, &dp);
        display_dp(t, s, &dp);
        display_longest_super_sequence(t, s, &dp);
        println!();
        

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
