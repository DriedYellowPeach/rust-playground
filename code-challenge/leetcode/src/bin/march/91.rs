
fn main() {
    println!("find the number of decoding path!");
}

fn num_decodings(s: String) -> i32 {
    let s = s.as_bytes();
    let valid = |token: &[u8]| -> i32 {
        if token[0] == b'0' {
            return 0
        }

        let mut val = 0;

        for v in token {
            val =  val * 10 + (*v - b'0') as i32
        }

        if val <= 26 { 1 } else { 0 }
    };
    let mut a = valid(&s[..1]);
    let mut b = a * valid(&s[1..2]) + valid(&s[..2]);

    for i in 2..s.len() {
        let cur = b * valid(&s[i..i+1]) + a * valid(&s[i-1..i+1]);
        (a, b) = (b, cur);
    }

    return b 
}

#[test]
fn test_num_decodings() {
    let s = "286".to_string();
    assert_eq!(num_decodings(s), 1 );
}

#[test]
fn test_valid() {
    let valid = |token: &[u8]| -> i32 {
        if token[0] == b'0' {
            return 0
        }

        let mut val = 0;

        for v in token {
            val =  val * 10 + (*v - b'0') as i32
        }

        if val <= 26 { 1 } else { 0 }
    };

    assert_eq!(valid(&vec![b'0', b'1']), 0);
    assert_eq!(valid(&vec![b'0']), 0);
    assert_eq!(valid(&vec![b'9']), 1);
    assert_eq!(valid(&vec![b'1', b'2']), 1);
    assert_eq!(valid(&vec![b'2', b'6']), 1);
    assert_eq!(valid(&vec![b'2', b'0']), 1);
    assert_eq!(valid(&vec![b'2', b'7']), 0);
}