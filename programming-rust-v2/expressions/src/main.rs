fn main() {
    println!("Hello, world!");
}

#[test]
fn test_semicolon_in_match() {
    let code = 5i32;

    let y = match code {
        0 => println!("hello"),
        _ => println!("world")
    };

    match code {
        0 => 0, 
        _ => 1
    };
}
