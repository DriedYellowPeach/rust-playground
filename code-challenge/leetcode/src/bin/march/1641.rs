fn main() {
    unimplemented!();
}

fn count_vowel_strings(n: i32) -> i32 {
    if n == 1 {
        return 5;
    }

    let mut n = n;
    let mut cur = vec![1; 5];
    let mut next = vec![0; 5];

    while n > 1 {
        let mut sum = 0;
        for (i, it) in cur.iter().enumerate() {
            sum += it;
            next[i] = sum;
        }
        std::mem::swap(&mut cur, &mut next);
        n -= 1;
    }

    cur.iter().sum()
}

#[test]
fn test_count_vowel_strings() {
    assert_eq!(count_vowel_strings(1), 5);
    assert_eq!(count_vowel_strings(2), 15);
    assert_eq!(count_vowel_strings(33), 66045);
}


