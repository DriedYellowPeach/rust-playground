
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut profit = 0;
    let mut min = prices[0];

    for price in prices {
        if price < min {
            min = price;
        }

        if price - min > profit {
            profit = price - min;
        }
    }

    profit
}

#[test]
fn test_max_profit() {
    let prices = vec![7, 1 ,5, 3, 6, 4];
    assert_eq!(max_profit(prices), 5);

    let prices = vec![7, 6, 5, 4, 3, 2, 1];
    assert_eq!(max_profit(prices), 0);
}
