
fn main() {
    unimplemented!();
}

fn check(s: &[u8], t: &[u8]) -> i32 {
    let mut cnt = 0;
    for i in 0..s.len() {
        cnt += (s[i] != t[i]) as i32;
        if cnt > 1 {
            break
        }
    }

    if cnt == 1 { 1 } else { 0 }
}

fn count_substrings(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut out = 0;

    for l in 1..=s.len() {
        // let mut i = 0;
        // while i + l <= s.len() {
        //     for j in 0..t.len() {

        //     }
        //     i += 1;
        // }
        for i in 0..=s.len() - l {
            for j in 0..=t.len() - l {
                out += check(&s[i..i + l], &t[j..j + l]);
                // println!("{} {} {} {}", i, j, l, out);
            }
        }
    }

    out
}

#[test]
fn test_count() {
    let s = "aba".to_string();
    let t = "baba".to_string();
    assert_eq!(count_substrings(s, t), 6);

    let s = "a".to_string();
    let t = "a".to_string();
    assert_eq!(count_substrings(s, t), 0);
}
