use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("hello day 3");
}

fn get_line_priority_sum(items: &str) -> usize {
    let mut ret = 0;
    let items = items.as_bytes();
    let half_len = items.len() / 2;
    let mut seen = [false; 26 * 2];
    let alpha_idx = |c: u8| -> usize {
        match c {
            b'a'..=b'z' => (c - b'a') as usize,
            b'A'..=b'Z' => (c - b'A') as usize + 26,
            _ => panic!("input format error"),
        }
    };

    for (i, &c) in items.iter().enumerate() {
        let idx = alpha_idx(c);
        if i < half_len {
            seen[idx] = true;
        } else if seen[idx] {
            seen[idx] = false;
            // print!("c is {}. ", c as char);
            ret += idx + 1;
        }
    }

    // println!("ret is {ret}");
    ret
}

fn get_all_priority_sum(path: &str) -> usize {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("can't open file: {e:?}");
        }
    };

    let mut priority_sum = 0;

    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                panic!("can't read line: {e:?}");
            }
        };

        priority_sum += get_line_priority_sum(&line);
    }

    priority_sum
}

fn get_group_badges(path: &str) -> usize {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            panic!("can't open file: {e:?}");
        }
    };
    let alpha_idx = |c: u8| -> usize {
        match c {
            b'a'..=b'z' => (c - b'a') as usize,
            b'A'..=b'Z' => (c - b'A') as usize + 26,
            _ => panic!("input format error"),
        }
    };
    let mut priority_sum = 0;
    // use bit map trick here:
    // set first member: i |= 0x1
    // set second member: i |= 0x1 << 1
    // set third member: i |= 0x1 << 2
    let mut seen = [0u8; 26 * 2];

    for (i, line) in BufReader::new(file).lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                panic!("can't read line: {e:?}");
            }
        };
        let member_idx = i%3;

        for &c in line.as_bytes() {
            let cnt = &mut seen[alpha_idx(c)];
            *cnt |= 1<<member_idx;
            // *cnt = (*cnt).clamp(0, member_idx + 1);
        }

        if member_idx == 2 {
            for (i, &cnt) in seen.iter().enumerate() {
                if cnt == 0b111 {
                    // println!("group is {i}");
                    priority_sum += i + 1;
                    break;
                }
            }
            seen = [0; 26 * 2];
        }
    }

    priority_sum
}

#[test]
fn test_get_priority_sum() {
    assert_eq!(get_all_priority_sum("input/day3_input_example.txt"), 157);
    println!("answer is {}", get_all_priority_sum("input/day3_input.txt"));
}

#[test]
fn test_get_group_badges() {
    assert_eq!(get_group_badges("input/day3_input_example.txt"), 70);
    println!("answer is {}", get_group_badges("input/day3_input.txt"));
}

#[test]
fn test_bit_map() {
    let mut i = 0u8;
    i |= 0x1;
    assert_eq!(i | 0b1, 0b1);
    i |= 0x1 << 1;
    assert_eq!(i | 0b1, 0b11);
    i |= 0x1 << 2;
    assert_eq!(i | 0b1, 0b111);
}
