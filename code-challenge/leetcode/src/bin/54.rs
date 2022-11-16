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
        let towards = (0, 0);
        // use Direction::*;
        // let direction = vec![Right, Down, Left, Up].into_iter().cycle();

        SpiralIter {
            width,
            height,
            position,
            towards,
            directions,
            matrix,
        }
    }

    fn on_corner(&self) -> bool {
        let y = self.position.1;
        let x = self.position.0;

        // corner one
        if x == y {
            return true
        }

        // corner two
        if x + y == self.width - 1 {
            return true
        }

        // corner three
        if x == y + 1 {
            return true
        }

        // corner four
        if x + y == self.height - 1 {
            return true
        }

        // corner five
        if x + 1 == y { 
            return true
        }

        return false

        // match (x, y) {
        //     (0, n) if n == 0 || n == self.height-1 => { true },
        //     (n, 0) if n == 0 || n == self.width-1 => { true },
        //     _ => { false }
        // }
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
fn test_spiral_order() {
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for i in 0..5 {
        matrix.push((0+6*i..6+6*i).collect())
    }
    println!("{matrix:?}");
   
    use Direction::*;
    let directions = vec![Right, Down, Left, Up].into_iter().cycle();
    let mut sp_iter = SpiralIter::new(&matrix, directions);
    // println!("{:?}", sp_iter.collect::<&i32>())
    for i in (0..15) {
        println!("{} ", sp_iter.next().unwrap());
        print!("{:?} ", sp_iter.towards);
    }
}