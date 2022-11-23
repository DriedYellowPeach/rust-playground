
fn main() {
    println!("partition string into palindrome, and find all the partition solution!");
}

fn get_palindrome_tbl(s: &str) -> Vec<Vec<bool>> {
    let s = s.as_bytes();
    let length = s.len();
    let mut tbl: Vec<Vec<bool>> = vec![vec![true; length]; length];

    for i in (0..length).rev() {
        for j in i+1..length {
            tbl[i][j] = (s[i] == s[j]) && (tbl[i+1][j-1]);
        }
    }

    return tbl
}

#[test]
fn test_get_palindrome_tbl() {
    let s = "aba";
    println!("{:#?}", get_palindrome_tbl(s));
}

fn partition(s: String) -> Vec<Vec<String>> {
    let tbl = get_palindrome_tbl(&s);
    let dp: Vec<Vec<String>> = Vec::new();

    
    return Vec::new();
}