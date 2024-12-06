pub struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let last_item = digits.len() - 1;
        let mut digits = digits;
        let mut key = 0;
        while key == 0 {
            if digits[last_item] < 9 {
                let mut digits = digits.clone();
                digits[last_item] += 1;
                key=1
            }
            else {
                digits[last_item] = 0;
            }
        }
        digits
    }
}

fn main() {
    let digits = vec![9,9];
    let result = Solution::plus_one(digits);
    println!("{:?}", result);
}
