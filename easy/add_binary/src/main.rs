pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;

        let mut len_a = a.len() as isize - 1;
        let mut len_b = b.len() as isize - 1;

        while len_a >= 0 || len_b >= 0 || carry > 0 {

            let bit_a = if len_a >= 0 {
                a.chars().nth(len_a as usize).unwrap().to_digit(2).unwrap()
            } else {
                0
            };

            let bit_b = if len_b >= 0 {
                b.chars().nth(len_b as usize).unwrap().to_digit(2).unwrap()
            } else {
                0
            };

            let sum = bit_a + bit_b + carry;

            result.push(if sum % 2 == 0 { '0' } else { '1' });
            carry = sum / 2;


            len_a -= 1;
            len_b -= 1;
        }

        result.chars().rev().collect()
    }
}

fn main() {
    let a = "01".to_string();
    let b = "1".to_string();

    let result = Solution::add_binary(a, b);
    println!("result: {}", result);
}
