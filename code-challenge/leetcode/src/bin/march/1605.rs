
fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn restore_matrix(row_sum: Vec<i32>, col_sum: Vec<i32>) -> Vec<Vec<i32>> {
    // for element of row, col, at most be min(row_sum[row], col_sum[col])
    // left_row_sum - element left_col_sum - element
    let mut row_sum = row_sum;
    let mut col_sum = col_sum;

    let (rows, cols) = (row_sum.len(), col_sum.len());
    let mut out = vec![vec![0; cols]; rows];

    for row in 0..rows {
        if row_sum[row] == 0 {
            continue
        }
        for col in 0..cols {
            if col_sum[col] == 0 {
                continue
            }
            let element = std::cmp::min(row_sum[row], col_sum[col]);
            out[row][col] = element;
            row_sum[row] -= element;
            col_sum[col] -= element;
        }
    }

    out
}

#[test]
fn test_restore_matrix() {
    let row_sum = vec![3, 8];
    let col_sum = vec![4, 7];

    assert_eq!(
        restore_matrix(row_sum, col_sum),
        vec![vec![3, 0], vec![1, 7]]
    );

    let row_sum = vec![5, 7, 10];
    let col_sum = vec![8, 6, 8];
    assert_eq!(
        restore_matrix(row_sum, col_sum),
        vec![vec![5, 0, 0], vec![3, 4, 0], vec![0, 2, 8]]
    );
}
