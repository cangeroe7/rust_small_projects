use std::vec;
use std::cmp;
fn max_profit(prices: Vec<i32>) -> i32 {
    let mut msf = i32::MAX;
    let mut ans = 0;

    for &p in &prices {
        msf = cmp::min(p, msf);
        ans = cmp::max(ans, p-msf);
    } 
    ans
}

fn main() {
    let prices: Vec<i32> = vec![7,1,5,3,6,4];
    let max_profit = max_profit(prices);
    println!("Your max profit: {}", max_profit);
}