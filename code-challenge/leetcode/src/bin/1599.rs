fn main() {}

fn min_operation_max_profix(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
    let mut cnt = 0;
    let mut waiting = 0;
    let running_profit = |boarding| -> i32 { boarding * boarding_cost - running_cost };
    let mut max_profit = i32::MIN;
    let mut cnt_record = 0;
    let mut profit = 0;

    if running_profit(4) < 0 {
        return -1;
    }

    loop {
        if cnt < customers.len() {
            waiting += customers[cnt];
        }
        let boarding = if waiting > 4 { 4 } else { waiting };
        waiting -= boarding;
        profit += running_profit(boarding);
        cnt += 1;
        if profit > max_profit {
            max_profit = profit;
            cnt_record = cnt;
        }

        if waiting == 0 && cnt >= customers.len() {
            break;
        }
    }

    if max_profit <= 0 {
        return -1;
    }

    cnt_record as i32
}

#[test]
fn test_profit() {
    let customers = vec![8, 3];
    let boarding_cost = 5;
    let running_cost = 6;

    assert_eq!(min_operation_max_profix(customers, boarding_cost, running_cost), 3);

    let customers = vec![10, 9, 6];
    let boarding_cost = 6;
    let running_cost = 4;

    assert_eq!(min_operation_max_profix(customers, boarding_cost, running_cost), 7);
}
