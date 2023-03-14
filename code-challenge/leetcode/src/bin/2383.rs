fn main() {
    unimplemented!();
}

fn min_number_of_hours(
    initial_energy: i32,
    initial_experience: i32,
    energy: Vec<i32>,
    experience: Vec<i32>,
) -> i32 {
    let mut energy_workout = energy.iter().sum::<i32>() - initial_energy + 1;
    if energy_workout < 0 {
        energy_workout = 0;
    }

    let mut exp_workout = 0;
    let mut exp_sum = 0;
    let mut exp_need = 0;

    for e in experience {
        exp_need = std::cmp::max(exp_need, e - exp_sum + 1);
        exp_sum += e;
    }

    exp_workout = exp_need - initial_experience;
    if exp_workout < 0 {
        exp_workout = 0;
    }

    energy_workout + exp_workout
}

#[test]
fn test_hours() {
    let energy = vec![1, 4, 3, 2];
    let experience = vec![2, 6, 3, 1];
    let initial_energy = 5;
    let initial_experience = 3;

    assert_eq!(min_number_of_hours(initial_energy, initial_experience, energy, experience), 8);
}
