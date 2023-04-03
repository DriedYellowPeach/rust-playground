use std::collections::HashMap;

fn main() {
    unimplemented!();
}

#[derive(PartialEq, Eq, Hash)]
enum Idx {
    Sentry,
    Norm(usize),
}

fn find_longest_subarray(array: Vec<String>) -> Vec<String> {
    // gap_map =>  {gap: index}
    let mut gap_map = HashMap::<i32, Idx>::new();
    gap_map.insert(0, Idx::Sentry);

    let mut len_max = 0;
    let mut out_range = 0usize..0usize;
    let mut gap = 0;

    // loop : two things to do
    // if gap not exist, insert, don't update, keep the smallest index with specific gap
    // find the same gap, v is the idx, range is => (v+1..=j) length is j-v
    for (i, ch) in array.iter().enumerate() {
        match ch.chars().next() {
            Some(ch) if ch.is_ascii_alphabetic() => {
                gap += 1;
            },
            Some(ch) if ch.is_ascii_digit() => {
                gap -= 1;
            },
            _ => panic!("invalid ouput")
        }

        if let Some(idx) = gap_map.get(&gap) {
            let start: usize = match idx {
                Idx::Sentry => 0,
                Idx::Norm(x) => x+1,
            };
            // so the range is (start..=i)
            let range_len = i - start + 1;
            if range_len > len_max {
                len_max = range_len;
                out_range = start..i+1;
            }
        } else {
            gap_map.insert(gap, Idx::Norm(i));
        }
    }

    array[out_range].to_vec()
}

#[test]
fn test_find_longest_subarray() {
    let array = ["A","1","B","C","D","2","3","4","E","5","F","G","6","7","H","I","J","K","L","M"]; 
    let array = array.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(find_longest_subarray(array), ["A","1","B","C","D","2","3","4","E","5","F","G","6","7"]);

    let array = ["A", "A"]; 
    let array = array.into_iter().map(|s| s.to_string()).collect();
    assert_eq!(find_longest_subarray(array), [] as [&str; 0]);
}
