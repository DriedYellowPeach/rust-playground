
fn main() {
    unimplemented!();
}

fn min_score_triangulation(values: Vec<i32>) -> i32 {
    let mut values = values;
    min_tri(&mut values)
}

#[test]
fn test_min_score_triangulation() {
    assert_eq!(min_score_triangulation(vec![1, 2, 3]), 6);
    assert_eq!(min_score_triangulation(vec![3, 7, 4, 5]), 144);
    assert_eq!(min_score_triangulation(vec![1, 3, 1, 4, 1, 5]), 13);
}


fn min_tri(values: &mut Vec<i32>) -> i32 {
    if values.len() == 3 {
        return values.iter().product()
    }

    let mut sum = 0;
    let l = values.len();
    // find max value idx
    let (max_idx, &big) = values.iter().enumerate().max_by_key(|a| a.1).unwrap();
    // contribute to sum
    sum += big * values[(max_idx + l - 1) % l] * values[(max_idx + 1) % l]; 
    // println!("sum: {}, max_idx: {}, big: {}", sum, max_idx, big);
    // swap and remove
    values.swap_remove(max_idx);
    // recursion
    sum += min_tri(values);

    sum 
}

