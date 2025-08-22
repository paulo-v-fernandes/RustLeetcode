pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut best = 0;
        
        for &price in prices.iter().skip(1) {  
            min = std::cmp::min(min, price);
            best = std::cmp::max(best, price - min);
        }
        best
    }
}

fn main() {
    let prices = vec![7,1,5,3,6,4];
    let result = Solution::max_profit(prices);
    println!("Result = {}", result);
}
