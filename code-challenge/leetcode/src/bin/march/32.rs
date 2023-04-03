
fn main() {
    println!("find the lognest sub str with valid parentheses!");
}

fn longest_valid_parentheses(s: String) -> i32 {
    let mut ret = 0;
    let s = s.as_bytes();
    // dp[i][0] is (sum, length)
    let mut dp: Vec<Vec<(i32, i32)>> = vec![Vec::new(); s.len()];
    let paren_val = |x: u8| {
        match x {
            b'(' => { -1 },
            b')' => { 1 },
            _ => { 0 }
        }
    };

    if s[0] == b'(' {
        dp[0].push((-1, 1));
    }

    for i in 1..dp.len() {
        let (front, back) = dp.split_at_mut(i);
        let (prev, cur) = (front.last().unwrap(), back.first_mut().unwrap());
        if s[i] == b'(' {
            cur.push((-1, 1))
        }

        for &(sum, length) in prev {
            let cur_sum = sum + paren_val(s[i]);
            if cur_sum == 0 {
                ret = std::cmp::max(ret, length+1);
            }
            if cur_sum <= 0 {
                cur.push((cur_sum, length+1));
            }
        }
    }

    return ret;
}

#[test]
fn test_longest_valid_parenthesis() {
    let s = "())(())()(".to_string();
    assert_eq!(longest_valid_parentheses(s), 6 );
}