// description: impl an iter for two-dimension vector which gives item by spiral order

use std::cmp::max_by_key;

fn main() {}

#[derive(Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl Direction {
    fn dir_vec(self) -> (i32, i32) {
        match self {
            Self::Right => (1, 0),
            Self::Left => (-1, 0),
            Self::Up => (0, -1),
            Self::Down => (0, 1),
        }
    }
}

struct SpiralIter<'a, T, I>
where
    I: Iterator<Item = Direction>,
{
    width: i32,
    height: i32,
    loop_cnt: i32,
    position: (i32, i32),
    towards: (i32, i32),
    directions: I,
    matrix: &'a Vec<Vec<T>>,
}

impl<'a, T, I> SpiralIter<'a, T, I>
where
    I: Iterator<Item = Direction>,
{
    fn new(matrix: &'a Vec<Vec<T>>, directions: I) -> SpiralIter<'a, T, I> {
        let width = matrix[0].len() as i32;
        let height = matrix.len() as i32;
        let position = (0, 0);
        let towards = Direction::Right.dir_vec();
        let loop_cnt = 0;
        // use Direction::*;
        // let direction = vec![Right, Down, Left, Up].into_iter().cycle();

        SpiralIter {
            width,
            height,
            loop_cnt, 
            position,
            towards,
            directions,
            matrix,
        }
    }

    fn on_corner(&self) -> bool {
        let y = self.position.1;
        let x = self.position.0;

        if x == y {
            return false
            
        }

        if y == self.loop_cnt && x == self.width - self.loop_cnt - 1{
            return true
        }

        if y == self.height - self.loop_cnt - 1 {
            return x == self.loop_cnt || x == self.width- self.loop_cnt - 1
        }

        if x == self.loop_cnt && y == self.loop_cnt + 1{
            return true
        }

        return false

    }
}

impl<'a, T, I: Iterator<Item = Direction>> Iterator for SpiralIter<'a, T, I> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        // checkout bounds
        if self.position.0 < 0
            || self.position.0 > self.width
            || self.position.1 < 0
            || self.position.1 > self.height
        {
            return None;
        }

        let x = self.position.0 as usize;
        let y = self.position.1 as usize;
        let ret = Some(&self.matrix[y][x]);

        // check on corner
        if self.on_corner() {
            if y == (self.loop_cnt + 1) as usize {
                self.loop_cnt += 1;
            }
            self.towards = self.directions.next().unwrap().dir_vec();
        }
        self.position.0 += self.towards.0;
        self.position.1 += self.towards.1;

        return ret
        // jump or walk
        // if self.corner_cnt % 4 == 1 {

        // } else {
        //     self.position.0 += self.towards.0;
        //     self.position.1 += self.towards.1;
        // }
        

        // do next
    }
}

#[test]
fn test_on_corner() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..5 {
        matrix.push((0+6*i..6+6*i).collect())
    }
    println!("{matrix:?}");

    use Direction::*;
    let directions = vec![Right, Down, Left, Up].into_iter().cycle();
    let mut sp_iter = SpiralIter::new(&matrix, directions);

    sp_iter.position = (0, 0);
    assert_eq!(sp_iter.on_corner(), false);
    sp_iter.position = (5, 0);
    assert_eq!(sp_iter.on_corner(), true);
    sp_iter.position = (5, 4);
    assert_eq!(sp_iter.on_corner(), true);
    sp_iter.position = (0, 1);
    assert_eq!(sp_iter.on_corner(), true);

    sp_iter.loop_cnt += 1;
    sp_iter.position = (1, 2);
    assert_eq!(sp_iter.on_corner(), true);
    sp_iter.position = (4, 1);
    assert_eq!(sp_iter.on_corner(), true);
}

#[test]
fn test_spiral_order() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..5 {
        matrix.push((0+6*i..6+6*i).collect())
    }
    println!("{matrix:?}");
   
    use Direction::*;
    let directions = vec![Down, Left, Up, Right].into_iter().cycle();
    let mut sp_iter = SpiralIter::new(&matrix, directions);
    // println!("{:?}", sp_iter.collect::<&i32>())
    for i in (0..30) {
        print!("{:?}", sp_iter.position);
        println!("{} ", sp_iter.next().unwrap());
        println!("{:?}", sp_iter.on_corner());
        print!("{:?} ", sp_iter.towards);
    }
}