
#[allow(dead_code)]
fn split_letter_case(s: &str) {
    let (upper, lower): (Vec<_>, Vec<_>) = s.chars().partition(|c| c.is_uppercase());
    println!("upper: {upper:?}, lower: {lower:?}");
}

#[test]
fn test_split() {
    split_letter_case("HelloWorld");
}
