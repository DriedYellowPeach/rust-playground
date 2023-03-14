fn main() {}

#[allow(dead_code)]
fn brace_expansion_ii(expression: String) -> Vec<String> {
    let mut out = parse_expression(expression.as_bytes());
    out.sort();
    out
}

#[test]
fn test_brace_expansion() {
    let s = "a{b,c}".to_string();
    println!("{:?}", brace_expansion_ii(s));
    // assert_eq!(brace_expansion_ii(s), ["ab", "ac"]);
    let s = "a{b,c}d{e,f}".to_string();
    println!("{:?}", brace_expansion_ii(s));
    // assert_eq!(brace_expansion_ii(s), ["abde","abdf", "acde", "acdf"]);
    let s = "a{b,{g,h}}d{e,f}".to_string();
    println!("{:?}", brace_expansion_ii(s));

    let s = "a".to_string();
    println!("{:?}", brace_expansion_ii(s));

    let s = "{a}".to_string();
    println!("{:?}", brace_expansion_ii(s));
}

use std::collections::HashSet;
// expression <= [char|brace]*
fn parse_expression(s: &[u8]) -> Vec<String> {
    let mut i = 0;
    let mut out = vec![String::new()];
    while i < s.len() {
        match s[i] {
            ch @ b'a'..=b'z' => {
                out.iter_mut().map(|it| it.push(ch as char)).last();
                i += 1;
            }
            b'{' => {
                let right = match_brace(s, i);
                let choices = parse_brace(&s[i + 1..right]);
                out = concat_choices(out, choices);
                i = right + 1;
            }
            _ => panic!("input invalid"),
        }
    }

    out
}

#[test]
fn test_parse_expression() {
    let s = b"a{b,c}";
    // let s = b"a";
    assert_eq!(parse_expression(s), ["ab", "ac"]);
}

// brace <= {[expression,]*}
fn parse_brace(s: &[u8]) -> HashSet<String> {
    let mut choices = HashSet::new();
    let mut i = 0;
    let mut start = 0;
    while i < s.len() {
        match s[i] {
            b'a'..=b'z' => {
                /* acc */
                i += 1;
            }
            b',' => {
                /* add choice */
                for item in parse_expression(&s[start..i]) {
                    choices.insert(item);
                }
                i += 1;
                start = i;
            }
            b'{' => {
                /* match_brace and skip all , inside */
                let right = match_brace(s, i);
                i = right + 1;
            }
            _ => panic!("invalid input"),
        }
    }
    for item in parse_expression(&s[start..]) {
        choices.insert(item);
    }

    choices
}

// Cartesian product
fn concat_choices(items: Vec<String>, choices: HashSet<String>) -> Vec<String> {
    let mut out = Vec::<String>::new();
    for item in items {
        for choice in &choices {
            let mut tmp = item.clone();
            tmp.push_str(choice);
            out.push(tmp);
        }
    }
    out
}

fn match_brace(s: &[u8], left: usize) -> usize {
    if s[left] != b'{' {
        panic!("not left brace");
    }

    let mut cnt = 0;
    let mut out = 0;

    for (i, item) in s.iter().enumerate().skip(left) {
        match item {
            b'{' => cnt += 1,
            b'}' => cnt -= 1,
            _ => {}
        }
        if cnt == 0 {
            out = i;
            break;
        }
    }

    if cnt != 0 {
        println!("s: {} left: {left} cnt:{cnt}", String::from_utf8_lossy(s));
        panic!("brace can't match, invalid input");
    }

    out
}

#[test]
fn test_match_brace() {
    let s = b"abc{{de}, {fg}}h";
    assert_eq!(match_brace(s, 3), 14);
}
