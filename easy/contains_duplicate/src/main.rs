use std::collections::HashSet;


impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for elem in nums{
            if !set.insert(elem) {
                return true
            }
        }
        false
    }
}
pub struct Solution;

fn main() {
    let nums = vec![1,1,2,2,3,4,5];
    let res = Solution::contains_duplicate(nums);
    println!("{}", res);
}
