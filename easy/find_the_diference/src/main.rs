impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut xor = 0u8;
        for c in s.chars() {
            xor ^= c as u8;
        }
        for c in t.chars() {
            xor ^= c as u8;
        }
        
        xor as char  
    }
}
pub struct Solution;
fn main() {
    let t = "abcde".to_string();
    let s = "abcd".to_string();
    let result = Solution::find_the_difference(s,t);
    println!("{}", result);
}
