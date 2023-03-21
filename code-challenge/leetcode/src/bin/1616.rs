
fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn check_palindrome_formation(a: String, b: String) -> bool {
    mix_panlindrome_check(&a, &b) || mix_panlindrome_check(&b, &a)
}

fn mix_panlindrome_check(a: &str, b: &str) -> bool {
    let sa = a.as_bytes();
    let sb = b.as_bytes();

    let mut left = 0;
    let mut right = sa.len() - 1;
    // let mut attach = 'b';
    let mut is_split = false;

    while left < right {
        if !is_split {
            if sa[left] == sb[right] {
                left += 1;
                right -= 1;
            } else {
                // recheck
                is_split = true;
            }
        } 

        if is_split {
            return check_palindrome(sa, left, right) || check_palindrome(sb, left, right)
        }
    }

    true
}

fn check_palindrome(s: &[u8], left: usize, right: usize) -> bool {
    let mut left = left;
    let mut right = right;

    while left < right {
        if s[left] != s[right] {
            return false
        }
        left += 1;
        right -= 1;
    }

    true
}

#[test]
fn test_mix_panlindrom_check() {
    let a = "x";
    let b = "y";

    assert!(mix_panlindrome_check(a, b));

    let a = "abdef";
    let b = "fecab";

    assert!(mix_panlindrome_check(b, a));

    let a = "ulacfd";
    let b = "jizalu";

    assert!(mix_panlindrome_check(a, b));
}
