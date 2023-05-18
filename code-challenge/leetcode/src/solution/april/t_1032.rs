
#[allow(dead_code)]
fn prev_perm_opt1(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;

    if arr.len() == 1 {
        return arr;
    }

    let mut j = (arr.len() - 2) as isize;
    while j > 0 && arr[j as usize] <= arr[j as usize + 1] {
        j -= 1;
    }

    if j == -1 {
        return arr;
    }

    let mut i = (arr.len() - 1) as isize;
    while i > j && arr[i as usize] >= arr[j as usize] {
        i -= 1;
    }

    while i > j && arr[i as usize] == arr[i as usize - 1] {
        i -= 1;
    }

    arr.swap(i as usize, j as usize);

    arr
}

#[test]
fn test_prev_perm_opt1() {
    assert_eq!(prev_perm_opt1(vec![3, 2, 1]), vec![3, 1, 2]);
    assert_eq!(prev_perm_opt1(vec![1, 1, 5]), vec![1, 1, 5]);
    assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7]), vec![1, 7, 4, 6, 9]);
    assert_eq!(prev_perm_opt1(vec![1, 9, 4, 6, 7, 9]), vec![1, 7, 4, 6, 9, 9]);
    assert_eq!(prev_perm_opt1(vec![3, 1, 1, 3]), vec![1, 3, 1, 3]);
}
