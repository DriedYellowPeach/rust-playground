fn main() {}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // if intervals.len < 2 return
    if intervals.len() < 2 {
        return intervals;
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|i| i[0]);

    let mut idx = 1;
    let mut ret = Vec::new();
    let mut buf = intervals.first().unwrap().clone();

    while idx < intervals.len() {
        let cur = &intervals[idx];
        if buf[1] < cur[0] {
            ret.push(buf.clone());
            buf[0] = cur[0];
            buf[1] = cur[1];
        } else {
            buf[1] = std::cmp::max(buf[1], cur[1]);
        }
        idx += 1;
    }
    ret.push(buf.clone());
    ret
}

fn merge_v2(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // if intervals.len < 2 return
    if intervals.len() < 2 {
        return intervals;
    }

    let mut intervals = intervals;
    intervals.sort_by_key(|i| i[0]);

    let mut ret = Vec::new();
    let mut iterval = intervals.into_iter();
    let mut buf = iterval.next().unwrap();

    for it in iterval {
        if buf[1] < it[0] {
            ret.push(buf);
            buf = it;
        } else {
            buf[1] = std::cmp::max(buf[1], it[1]);
        }
    } 
    ret.push(buf);
    ret
}

#[test]
fn test_merge() {
    let intervals = [[1, 3], [2, 6], [8, 10], [15, 18]];
    assert_eq!(merge_v2(intervals.into_iter().map(|it| vec![it[0], it[1]]).collect()), [[1, 6], [8, 10], [15, 18]]);
}
