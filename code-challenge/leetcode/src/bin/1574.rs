fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
    // first find the longest ascend postfix
    let mut j = arr.len() - 1;
    let mut out;

    while j > 0 && arr[j - 1] <= arr[j] {
        j -= 1;
    }

    out = j;

    let mut i = 0;
    while i < j {
        while j < arr.len() && arr[i] > arr[j] {
            j += 1;
        }

        out = std::cmp::min(out, j - i - 1);

        if i + 1 < arr.len() && arr[i + 1] < arr[i] {
            break;
        }
        i += 1;
    }

    out as i32
}

#[test]
fn test_find_length_of_shortest_subarray() {
    let arr = vec![1, 2, 3, 10, 4, 2, 3, 5];
    let out = find_length_of_shortest_subarray(arr);
    assert_eq!(out, 3);
    let arr = vec![5, 4, 3, 2, 1];
    let out = find_length_of_shortest_subarray(arr);
    assert_eq!(out, 4);
    let arr = vec![1, 2, 3];
    let out = find_length_of_shortest_subarray(arr);
    assert_eq!(out, 0);
    let arr = vec![1];
    let out = find_length_of_shortest_subarray(arr);
    assert_eq!(out, 0);
}

