pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let cleaned: String = s.chars()
            .filter(|c| c.is_alphanumeric())  
            .map(|c| c.to_ascii_lowercase())  
            .collect();

        let reverse: String = cleaned.chars().rev().collect();

        println!("Original: {}", cleaned);
        println!("Invertida: {}", reverse);

        if reverse == cleaned {
            return true;  
        }
        false  
    }
}

fn main() {
    let word = "A man, a plan, a canal: Panama".to_string(); 
    let is_pa: bool = Solution::is_palindrome(word);
    println!("Result: {}", is_pa);
}
