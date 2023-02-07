fn main() {
    println!("hello day4");
}

fn parse_section_pairs(line: &str) -> [i32;4] {
    let mut split = line.split(',');
    let nums1 = split.next().unwrap().split('-');
    let nums2 = split.next().unwrap().split('-');
    let mut ret = [0; 4];

    for (i, s) in nums1.chain(nums2).enumerate() {
        ret[i] = s.parse::<i32>().unwrap_or_else(|e| panic!("input format invalide: {e}") );
    }

    ret 
}

use aoc::ioutils::get_lines;
fn fully_contain_pairs_count(path: &str) -> i32 {
    let mut cnt = 0;
    for line in get_lines(path) {
        let pairs = parse_section_pairs(&line);
        let (x1, x2, x3, x4) = (pairs[0], pairs[1], pairs[2], pairs[3]);
        if (x1 <= x3 && x2 >= x4) || (x3 <= x1 && x4 >= x2) {
            // println!("({x1}, {x2}), ({x3}, {x4})");
            cnt += 1;
        }
    }
    cnt
}

fn overlap_pairs_count(path: &str) -> i32 {
    let mut cnt = 0;
    for line in get_lines(path) {
        let pairs = parse_section_pairs(&line);
        let (a1, a2, b1, b2) = (pairs[0], pairs[1], pairs[2], pairs[3]);
        if b1 <= a2 && b2 >= a1 {
            cnt += 1;
        }
    }
    cnt
}

#[test]
fn test_string_split() {
    let mut split = "1,2,3".split(',');
    assert_eq!(split.next(), Some("1"));
    assert_eq!(split.next(), Some("2"));
    assert_eq!(split.next(), Some("3"));
}

#[test]
fn test_chain() {
    let v = vec![5, 6, 7];
    let a = [1, 2, 3, 4];

    for i in a.iter().chain(v.iter()) {
        println!("{i}");
    }
}

#[test]
fn test_parse_section_pairs() {
    assert_eq!(parse_section_pairs("1-2,3-4"), [1, 2, 3, 4]);
    assert_eq!(parse_section_pairs("1-7,3-8"), [1, 7, 3, 8]);
}

#[test]
fn test_fully_contain_pairs_count() {
    assert_eq!(fully_contain_pairs_count("input/day4_input_example.txt"), 2);
    println!("answer is {}", fully_contain_pairs_count("input/day4_input.txt"));
}

#[test]
fn test_overlap_pairs_count() {
    assert_eq!(overlap_pairs_count("input/day4_input_example.txt"), 4);
    println!("answer is {}", overlap_pairs_count("input/day4_input.txt"));
}
