pub struct Solution;


impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut aux: Vec<i32> = vec![];
        for i in 0..=row_index {
            let mut row: Vec<i32> = vec![];

            if i == 0 {
                row.push(1);
            } else if i == 1 {
                row.extend([1, 1]);
            } else {
                row.push(1); 
                for j in 1..i as usize {
                    row.push(aux[j - 1] + aux[j]);
                }
                row.push(1); 
            
        }
        aux = row.clone();
    }
    aux
    }
}


fn main() {
    let rows: i32 = 3;
    let result = Solution::get_row(rows);
    println!("{:?}", result);
}