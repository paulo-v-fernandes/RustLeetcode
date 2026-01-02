pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut majoritary = 0;

        for num in nums{
            if count == 0 {
                majoritary = num;
            }
            if majoritary != num{
                count -= 1 ;
            }else{
                count +=1;
            }
            }
            majoritary
               
    }
}


fn main() {
    let nums = vec![3,2,3];
    let result = Solution::majority_element(nums);
    println!("Response: {}", result);
}
