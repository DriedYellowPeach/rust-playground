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

#[test]
fn test_min_moves_to_seat() {
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    assert_eq!(min_moves_to_seat(seats, students), 4);
}
