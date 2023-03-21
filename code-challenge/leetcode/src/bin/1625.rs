fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
    let ss = format!("{s}{s}");
    let mut seen = vec![0; s.len()];
    let mut out = s.as_bytes().to_owned();


    let mut i = 0;
    while seen[i] == 0 {
        seen[i] = 1;
        for j in 0..10 {
            let k_limit = if b % 2 == 0 { 0 } else { 9 };
            for k in 0..=k_limit  {
                let mut new_s = ss[i..i+s.len()].as_bytes().to_owned();
                for p in (1..s.len()).step_by(2) {
                    new_s[p] = (((new_s[p] - b'0') as i32 + j * a) % 10) as u8 + b'0';
                }

                for p in (0..s.len()).step_by(2) {
                    new_s[p] = (((new_s[p] - b'0') as i32 + k * a) % 10) as u8 + b'0';
                }

                if new_s < out {
                    out = new_s;
                }
            }
        
        }
        i = (i + b as usize) % s.len();
    }

    String::from_utf8(out).unwrap()
}
