fn main() {
    println!("Hello, world!");
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy_idx = 0;
    let mut sell_idx = 1;
    let mut profit = 0;

    while sell_idx < prices.len() {
        let sell_price = prices[sell_idx];
        let buy_price = prices[buy_idx];

        if sell_price > prices[buy_idx] {
            profit = std::cmp::max(profit, sell_price - buy_price);
        } else {
            buy_idx = sell_idx;
        }
        sell_idx += 1;
    }
    profit
}
