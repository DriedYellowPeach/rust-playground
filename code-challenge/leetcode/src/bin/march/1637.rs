
fn main() {
    unimplemented!();
}

fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut out = 0;
    let mut points = points;

    points.sort_by_key(|x| x[0]);

    for i in 1..points.len() {
        out = std::cmp::max(out, points[i][0] - points[i-1][0]);
    }

    out
}

#[test]
fn test_max_width_of_vertical_area() {
    assert_eq!(max_width_of_vertical_area(vec![vec![8,7],vec![9,9],vec![7,4],vec![9,7]]), 1);
    assert_eq!(max_width_of_vertical_area(vec![vec![3,1],vec![9,0],vec![1,0],vec![1,4],vec![5,3],vec![8,8]]), 3);
}


