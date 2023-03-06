fn main() {}

fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let height = matrix.len();
    let width = matrix.first().unwrap().len();
    let mut loop_cnt = 0;
    let mut ret = Vec::new();
    let (mut px, mut py) = (0usize, 0usize);

    while ret.len() != width * height {
        // keep right, px+=1
        while px < width - loop_cnt - 1 {
            // yield matrix[py][px]
            ret.push(matrix[py][px]);
            px += 1;
        }

        // keep down, py += 1
        while py < height - loop_cnt - 1 {
            ret.push(matrix[py][px]);
            py += 1;
        }

        // keep left, px -= 1
        while px >= loop_cnt + 1 {
            ret.push(matrix[py][px]);
            px -= 1;
        }

        // keep up, py -= 1
        while py >= loop_cnt + 2 {
            ret.push(matrix[py][px]);
            py -= 1;
        }

        println!("{ret:?}");
        loop_cnt += 1;
    }

    ret
}

fn spiral_order_v2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (rows, columns) = (matrix.len(), matrix.first().unwrap().len());
    let mut ret = Vec::with_capacity(rows * columns);
    let (mut left_col, mut right_col) = (0isize, (columns - 1) as isize);
    let (mut top_row, mut bottom_row) = (0isize, (rows - 1) as isize);

    let mut append = |row, col| ret.push(matrix[row as usize][col as usize]);

    while left_col <= right_col && top_row <= bottom_row {
        for col in left_col..=right_col {
            append(top_row, col);
        }

        for row in top_row + 1..=bottom_row {
            append(row, right_col);
        }

        if left_col == right_col || top_row == bottom_row {
            break;
        }

        for col in (left_col..=right_col - 1).rev() {
            append(bottom_row, col);
        }

        for row in (top_row + 1..=bottom_row - 1).rev() {
            append(row, left_col);
        }
        left_col += 1;
        right_col -= 1;
        top_row += 1;
        bottom_row -= 1;
    }

    ret
}

fn get_matrix_layer(matrix: &Vec<Vec<i32>>, layer: usize) -> Vec<i32> {
    if matrix.len() <= 2 * layer || matrix.first().unwrap().len() <= 2 * layer {
        return Vec::new();
    }
    let height = matrix.len() - 2 * layer;
    let width = matrix.first().unwrap().len() - 2 * layer;
    let (xa, ya) = (layer, layer);
    let (xb, yb) = (xa + width - 1, ya + height - 1);
    let mut ret = Vec::new();

    // (xa, ya)-----(xb, ya)
    // |                   |
    // |                   |
    // (xa, yb)-----(xb, yb)

    ret.push(matrix[ya][xa]);

    let y = ya;
    for x in xa + 1..=xb - 1 {
        ret.push(matrix[y][x]);
    }

    if xa != xb {
        ret.push(matrix[ya][xb]);
    }

    let x = xb;
    for y in ya + 1..=yb - 1 {
        ret.push(matrix[y][x]);
    }

    if ya != yb {
        ret.push(matrix[yb][xb]);
    }

    let y = yb;
    for x in (xa + 1..=xb - 1).rev() {
        ret.push(matrix[y][x]);
    }

    if xa != xb {
        ret.push(matrix[yb][xa]);
    }

    let x = xa;
    for y in (ya + 1..=yb - 1).rev() {
        ret.push(matrix[y][x]);
    }

    ret
}

#[test]
fn test_get_layer() {
    let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    println!(
        "{:?}",
        get_matrix_layer(
            &matrix
                .into_iter()
                .map(|row| row.into_iter().collect())
                .collect(),
            0
        )
    );
}

#[test]
fn test_spiral_order() {
    let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    println!(
        "{:?}",
        spiral_order_v2(
            matrix
                .into_iter()
                .map(|row| row.into_iter().collect())
                .collect()
        )
    );
}

#[test]
fn test_range() {
    let x = 1;
    let y = 1;
    assert_eq!((x..y).next(), None);
}
