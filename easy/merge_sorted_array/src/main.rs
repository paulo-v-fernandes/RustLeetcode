impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        println!("nums1: {:?}", nums1);
        println!("nums2: {:?}", nums2);
        println!("n: {}", n);

        nums1.truncate(m as usize);
        nums1.extend_from_slice(nums2);
        nums1.sort();

        println!("Resultado: {:?}", nums1);
    }
}

pub struct Solution;

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2 = vec![2, 5, 6];
    let n = nums2.len() as i32;

    Solution::merge(&mut nums1, m, &mut nums2, n);
}
