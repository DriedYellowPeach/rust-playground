#[allow(dead_code)]
fn add_negbinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut sum =
        arr1.iter().fold(0, |acc, x| acc * -2 + x) + arr2.iter().fold(0, |acc, x| acc * -2 + x);

    if sum == 1 {
        return vec![1];
    }

    let mut ans = vec![];
    let mut rem;

    while sum != 1 {
        (sum, rem) = my_mod(sum, -2);
        ans.push(rem);
    }

    ans.push(1);

    ans.reverse();
    ans
}

#[test]
fn test_add_negbinary() {
    assert_eq!(
        add_negbinary(vec![1, 1, 0, 1, 0, 1], vec![1, 0, 0, 0, 0]),
        vec![1, 0, 1]
    );
    assert_eq!(
        add_negbinary(vec![1, 1, 0, 1, 0, 1], vec![0]),
        // vec![1, 0, 1, 0, 1, 1]
        vec![1, 1, 0, 1, 0, 1]
    );
}

fn my_mod(a: i32, b: i32) -> (i32, i32) {
    let quotient = a / b;
    let rem = a % b;

    if rem < 0 {
        // if a = -2 * b -1
        // then a = -2 * b -2 + 1
        // that's a = -2 * (b+1) + 1
        // so quotient is b+1, and rem is 1
        (quotient + 1, 1)
    } else {
        (quotient, rem)
    }
}

#[test]
fn math_mod_rules() {
    assert_eq!(-11 % -2, -1);
    assert_ne!(-11 % -2, 1);
    assert_eq!(-11 % 2, -1);
}

#[test]
fn test_my_mod() {
    assert_eq!(my_mod(-11, -2), (6, 1));
    assert_eq!(my_mod(11, -2), (-5, 1));
    // assert_eq!(my_mod(-11, 2), (6, 1));
}
