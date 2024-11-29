pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut inicio = 0;
        let mut fim = nums.len();
    
        while inicio < fim {
            let meio = inicio + (fim - inicio) / 2;
            if nums[meio] == target {
                return meio as i32;
            } else if nums[meio] < target {
                inicio = meio + 1;
            } else {
                fim = meio;
            }
        }
        inicio as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 9;
    let search = Solution::search_insert(nums, target);
    println!("search: {}", search);
}
