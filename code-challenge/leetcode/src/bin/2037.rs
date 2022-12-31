fn main() {
    println!("find seat");
}

fn min_moves_to_seat(seats: Vec<i32>, students: Vec<i32>) -> i32 {
    let mut seats = seats;
    let mut students = students;
    seats.sort();
    students.sort();

    let it = seats.iter().zip(&students);
    let mut ret = 0;

    for (&a, &b) in it {
        ret += if a > b { a - b} else { b - a };
    }

    ret
}
