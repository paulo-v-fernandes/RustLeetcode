use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        println!("vec: {:?}", nums);

        let mut aux_set: HashSet<i32> = HashSet::new();

        for elem in nums {
            if aux_set.contains(&elem) {
                aux_set.remove(&elem); 
            } else {
                aux_set.insert(elem); 
            }

            println!("Elemento atual: {}", elem);
            println!("Estado de aux_set: {:?}", aux_set);
        }


        *aux_set.iter().next().unwrap_or(&1)
    }
}

fn main() {
    let nums = vec![1, 1, 2];
    let result = Solution::single_number(nums);
    println!("Result: {}", result);
}
