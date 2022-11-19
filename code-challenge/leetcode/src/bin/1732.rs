
fn main() {
    println!("find the highest altitude!");
}

fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ret = 0;
    gain.iter()
        .fold(0, 
        |acc, v| {
            ret = std::cmp::max(acc + v, ret);
            acc + v
        });
    return ret
}

#[test]
fn test_largest_altitude() {
    let v = vec![-1, 3, 4, -5, -7];
    assert_eq!(largest_altitude(v), 6);
}