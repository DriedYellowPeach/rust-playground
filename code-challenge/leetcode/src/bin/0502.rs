
fn main() {
    unimplemented!();
}

fn print_bin(num: f64) -> String {
    let mut num = num;
    if num <= 0.0 || num >= 1.0 {
        panic!("input invalid");
    }
    let base = 1.0;
    let mut ret = "0.".to_string();

    while num != 0.0 &&  ret.len() < 32 {
        if num * 2.0 >= base {
            ret.push('1');
            num = num * 2.0 - base;
        } else {
            ret.push('0');
            num *= 2.0;
        }
    }
    // println!("{ret}");

    if num != 0.0 { "ERROR".to_string() } else { ret }
}

fn print_bin_from_string(num: String) -> String {
    String::new()
}

#[test]
fn test_print_bin() {
    assert_eq!(print_bin(0.625f64), "0.101");
    assert_eq!(print_bin(0.1), "ERROR");
}
