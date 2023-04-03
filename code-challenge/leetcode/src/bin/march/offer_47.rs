fn main() {
    unimplemented!();
}

fn max_value(grid: Vec<Vec<i32>>) -> i32 {
    let row = grid.len();
    let col = grid.first().unwrap().len();
    let mut out = vec![vec![0; col]; row];
    out[0][0] = grid[0][0];

    let valid = |x, y| x >= 0 && x < row as i32 && y >= 0 && y < col as i32;
    let set_out = |out: &mut Vec<Vec<i32>>, x: i32, y: i32, v: i32| {
        out[x as usize][y as usize] = std::cmp::max(grid[x as usize][y as usize] + v, out[x as usize][y as usize])
    };

    for i in 0..col {
        let mut c = i as i32;
        let mut r = 0;
        while valid(r, c) {
            let v = out[r as usize][c as usize];
            
            if valid(r, c + 1) {
                set_out(&mut out, r, c + 1, v);
            }

            if valid(r + 1, c) {
                set_out(&mut out, r + 1, c, v);
            }
            c -= 1;
            r += 1;
        }
    }

    for j in 1..row {
        let mut r = j as i32;
        let mut c = col as i32 - 1;
        while valid(r, c) {
            let v = out[r as usize][c as usize];

            if valid(r, c + 1) {
                set_out(&mut out, r, c + 1, v);
            }

            if valid(r + 1, c) {
                set_out(&mut out, r + 1, c, v);
            }
            c -= 1;
            r += 1;
        }
    }

    *out.last().unwrap().last().unwrap()
}

#[test]
fn test_max_value() {
    let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];
    assert_eq!(max_value(grid), 12);
}
