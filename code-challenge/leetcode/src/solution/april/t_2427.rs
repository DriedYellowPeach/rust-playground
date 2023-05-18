
fn common_factors(a: i32, b: i32) -> i32 {
    let common = gcd(a, b);
    let mut x = 0;
    let mut out = 0;
    while x * x <= common {
        if common % x == 0 {
            if x * x != common {
                out += 1;
            }
            out += 1;
        }
        x += 1;
    }

    out
}

fn gcd(m: i32, n: i32) -> i32 {
    assert!(m != 0 && n != 0);
    let mut m = m;
    let mut n = n;

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(4, 6), 2);
    assert_eq!(gcd(9, 21), 3);
}

