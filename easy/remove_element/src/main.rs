pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i = 0;
        let mut j = 0;

        while i < nums.len() {
            if nums[i] != val {
                nums[j] = nums[i];
                j += 1;
            }
            i += 1;
        }

        nums.truncate(j);
        j as i32
    }
}



fn main() {
    let mut nums = vec![3, 2, 2, 3,2,4,5,6,3,3,3,3,3];
    let val:i32 = 3;
    let nums_vec = Solution::remove_element(&mut nums, val);
    println!("New vector: {}", nums_vec);
}