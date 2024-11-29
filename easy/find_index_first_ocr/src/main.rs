pub struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        if needle.len() > haystack.len() {
            return -1;
        }
        
        println!("haystack: {}, needle: {}", haystack, needle);
        for i in 0..=haystack.len() - needle.len() {
            if needle == &haystack[i..i + needle.len()] {
                return i as i32;
            }
        }
        -1
    }
}

fn main() {
    let haystack = "a".to_string();
    let needle = "a".to_string();
    let back = Solution::str_str(haystack.clone(), needle.clone());
    println!("Resultado: haystack: {}, needle: {}, posição: {}", haystack, needle, back);
}
