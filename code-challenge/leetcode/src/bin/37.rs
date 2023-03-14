// solve sodoku
fn main() {}

const ROW: usize = 9;
const COL: usize = 9;

#[derive(Debug)]
struct Clue {
    idx: usize,
    val: char,
}

// return (row, col)
fn idx_as_xy(idx: usize) -> (usize, usize) {
    (idx / ROW, idx % ROW)
}

#[test]
fn test_idx_as_xy() {
    assert_eq!(idx_as_xy(9), (1, 0));
    assert_eq!(idx_as_xy(80), (8, 8));
}

fn next_dot(board: &Vec<Vec<char>>, idx: usize) -> Option<usize> {
    for i in idx..ROW * COL {
        let (r, c) = idx_as_xy(i);
        // println!("{} {} {} {} ", i, r, c, board[r][c]);
        if board[r][c] == '.' {
            return Some(i);
        }
    }
    None
}

#[test]
fn test_next_dot() {
    let board = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let board = board.into_iter().map(|it| it.to_vec()).collect();
    assert_eq!(next_dot(&board, 0), Some(2));
    assert_eq!(next_dot(&board, 2), Some(2));
    assert_eq!(next_dot(&board, 4), Some(5));
    assert_eq!(next_dot(&board, 9), Some(10));
}

fn check(board: &Vec<Vec<char>>, idx: usize, ch: char) -> bool {
    let (rows, cols) = idx_as_xy(idx);
    // check row
    let row_ok = board[rows].iter().all(|it| it != &ch);
    // check col
    let col_ok = (0..ROW).all(|it| board[it][cols] != ch);
    // check block
    let mut blk_ok = true;

    let blk_row = rows / 3;
    let blk_col = cols / 3;
    for r in blk_row * 3..blk_row * 3 + 3 {
        for c in blk_col * 3..blk_col * 3 + 3 {
            if board[r][c] == ch {
                blk_ok = false;
            }
        }
    }

    row_ok && col_ok && blk_ok
}

#[test]
fn test_check() {
    let board = [
        ['5', '3', '4', '6', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let board = board.into_iter().map(|it| it.to_vec()).collect();
    // assert_eq!(check(&board, 5, '8'), true);
    assert!(check(&board, 2, '1'));
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let mut stk = Vec::<Clue>::new();
    let idx = match next_dot(board, 0) {
        Some(x) => x,
        None => return,
    };
    stk.push(Clue { idx, val: '0' });

    loop {
        let top_clue = stk.last_mut().expect("invalid input, no solution");
        let mut found = false;
        let (row, col) = idx_as_xy(top_clue.idx);

        // find clue for top
        // set this first
        for c in (top_clue.val..='9').skip(1) {
            /* not '1'..='9' */
            if check(board, top_clue.idx, c) {
                top_clue.val = c;
                found = true;
                board[row][col] = c;
                break;
            }
        }

        if !found {
            board[row][col] = '.';
            stk.pop();
        } else {
            let next_idx = match next_dot(board, top_clue.idx + 1) {
                Some(x) => x,
                None => return,
            };
            let next_clue = Clue {
                idx: next_idx,
                val: '0',
            };
            stk.push(next_clue);
        }
    }
}

#[test]
fn test_solve_sudoku() {
    let board = [
        ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        ['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let mut board = board.into_iter().map(|it| it.to_vec()).collect();
    solve_sudoku(&mut board);
    board.iter().inspect(|it| println!("{:?}", it)).last();
}
