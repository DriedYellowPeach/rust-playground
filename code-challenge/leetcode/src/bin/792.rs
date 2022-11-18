// type: string

use std::collections::HashMap;

fn main() {

}

fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
    let s = s.as_bytes();
    let words = words.iter().map(|wds| wds.as_bytes()).collect::<Vec<&[u8]>>();
    let mut ret = 0;
    let mut ptrs = vec![0; words.len()];

    for c in s {
        for (i, wd) in words.iter().enumerate() {
            if ptrs[i] == wd.len() {
                continue
            }

            if wd[ptrs[i]] == *c {
                ptrs[i] += 1;
            }
        }
    }
    println!("{:?}", ptrs);
    let ret: usize= ptrs.iter()
                    .enumerate()
                    .fold(0, 
                        |acc, val| { 
                            let is_end = if *val.1 == words[val.0].len() { 1 } else { 0 };
                            acc + is_end 
                        }
                    );

    return ret as i32;
}

fn num_matching_subseq_bak(s: String, words: Vec<String>) -> i32 {
    let mut ret = 0;
    // let mut word_iters = words.iter().map(|wds| wds.chars().peekable());
    let mut word_iters: Vec<std::iter::Peekable<std::str::Chars>> = words.iter().map(|wds| wds.chars().peekable()).collect();
    for c in s.chars() {
        for w in &mut word_iters {
            match w.peek() {
                Some(&ch) if ch == c => { w.next(); },
                None => { ret += 1; },
                _ => { }
            };
        }        
    }

    
    return ret;
}

#[test]
fn test_nums_matching_seq() {
    let s = "abcdebd".to_string();
    let words = vec!["abcdebd".to_string(), "aa".to_string(), "b".to_string()];
    assert_eq!(num_matching_subseq(s, words), 2);
}

// optimization: &[u8] as key 
fn handle_seq(map: &mut HashMap<Vec<u8>, usize>, s: &String, seq: &str) -> bool {
    let mut needle = 0usize;
    let s = s.as_bytes();
    let seq = seq.as_bytes();

    for l in 1..seq.len() {
        let target = &seq[..l];
        let index = map.get(target);
        match index {
            Some(&i) => { needle = i + 1 },
            None => {
                // let next_needel = (&s[needle+1..]).bytes().position(|x| *x == )
                let next_needle = s[needle..].iter().position(|x| x == target.last().unwrap());
                match next_needle {
                    Some(nndl) => { 
                        needle = nndl + 1;
                        map.insert(target.to_vec(), nndl);
                    },
                    None => { return false }
                }
            }
        }
    }

    return false
}