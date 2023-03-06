fn main() {}

fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut items1 = items1;
    let mut items2 = items2;

    items1.sort_by_key(|item| item[0]);
    items2.sort_by(|a, b| a[0].cmp(&b[0]));

    let (mut i, mut j) = (0, 0);
    let mut ret = Vec::new();

    use std::cmp::Ordering;
    while i < items1.len() && j < items2.len() {
        match items1[i][0].cmp(&items2[j][0]) {
            Ordering::Less => {
                ret.push(items1[i].clone());
                i += 1;
            }
            Ordering::Equal => {
                ret.push(vec![items1[i][0], items1[i][1] + items2[j][1]]);
                (i, j) = (i + 1, j + 1);
            }
            Ordering::Greater => {
                ret.push(items2[j].clone());
                j += 1;
            }
        }
    }

    if i != items1.len() {
        ret.extend_from_slice(&items1[i..]);
    }

    if j != items2.len() {
        ret.extend_from_slice(&items2[j..])
    }

    ret
}

fn with_hash_map(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use std::collections::HashMap;
    let mut ret = items1
        .into_iter()
        .chain(items2)
        .fold(HashMap::new(), |mut acc, x| {
            let ent = acc.entry(x[0]).or_insert(0);
            *ent += x[1];
            acc
        })
        .into_iter()
        .map(|x| vec![x.0, x.1]).collect::<Vec<_>>();
    ret.sort_by_key(|x| x[0]);
    ret
}

#[test]
fn test_merge_similar_items() {
    let a = vec![vec![1, 1], vec![4, 5], vec![3, 8]];
    let b = vec![vec![3, 1], vec![1, 5], vec![7, 8]];

    // assert_eq!(
    //     merge_similar_items(a, b),
    //     vec![[1, 6], [3, 9], [4, 5], [7, 8]]
    // );
    assert_eq!(
        with_hash_map(a, b),
        vec![[1, 6], [3, 9], [4, 5], [7, 8]]
    );
}
