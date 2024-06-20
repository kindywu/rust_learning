fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min_price = i32::MAX;

    for &price in prices.iter() {
        if price < min_price {
            // 更新最小价格
            min_price = price;
        } else if price - min_price > max_profit {
            // 更新最大盈利
            max_profit = price - min_price;
        }
    }

    max_profit
}

fn main() {
    let prices = vec![90, 30, 50, 10, 60, 125, 88, 135, 78];
    let profit = max_profit(prices);
    assert_eq!(profit, 125);
    let prices = vec![125, 30, 50, 90, 60, 110];
    let profit = max_profit(prices);
    assert_eq!(profit, 80);
}
