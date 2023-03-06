
fn main() {

}

fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let length = grid.len();
    let mut local = vec![vec![0; length-2]; length-2];
    let max_in_matrix = |r: usize, c: usize| -> i32 {
        let mut max:i32 = grid[r][c];
        for i in r..r+3 {
            for j in c..c+3 {
                if grid[i][j] > max {
                    max = grid[i][j]
                } 
            }
        } 
        max
    };
    for i in 0..length-2 {
        for j in 0..length-2 {
            local[i][j] = max_in_matrix(i, j);
        }
    }
    local
}

#[test]
fn test_largest_local() {
    let array_to_vec = |arr: [[i32; 4]; 4]| -> Vec<Vec<i32>> {
        let mut ret = vec![vec![0; 4]; 4];
        for (ridx, r) in arr.iter().enumerate() {
            for (cidx, num) in r.iter().enumerate() {
                ret[ridx][cidx] = *num;
            }
        }

        ret
    };

    let grid = [[9,9,8,1],[5,6,2,6],[8,2,6,4],[6,2,2,2]];
    let grid = array_to_vec(grid);
    assert_eq!(largest_local(grid), [[9,9],[8,6]]);
}
