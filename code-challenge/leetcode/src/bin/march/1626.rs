use std::cmp::Ordering;


fn main() {
    unimplemented!();
}

fn best_team_score(scores: Vec<i32>, ages: Vec<i32>) -> i32 {
    let mut score_ages = scores.into_iter().zip(ages.into_iter()).collect::<Vec<_>>();

    score_ages.sort_by(|a, b| -> Ordering {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut dp = vec![0; score_ages.len()];

    dp[0] = score_ages[0].0;

    for i in 1..score_ages.len() {
        for j in 0..i {
            if score_ages[i].1 >= score_ages[j].1 {
                dp[i] = std::cmp::max(dp[i], dp[j]);
            }
        }
        dp[i] += score_ages[i].0
    }

    dp.into_iter().max().unwrap()
}
