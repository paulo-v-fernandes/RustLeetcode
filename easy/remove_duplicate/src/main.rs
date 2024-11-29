// Definição da estrutura Solution
pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {

        let mut i = 0;
        while i < nums.len() - 1 { 
            if nums[i] == nums[i + 1] {
                println!("I: {}, value: {}", i, nums[i]);
                nums.remove(i + 1); 
            } else {
                i += 1;  
            }
            println!("I: {}, value: {}", i, nums[i]);
        }

        println!("Updated vector: {:?}", nums);
        nums.len() as i32
    }
    
}

fn main() {
    let mut nums = vec![1, 1, 2];
    
    let nums_ref = Solution::remove_duplicates(&mut nums);
    
    println!("Updated vector: {}", nums_ref);
}
