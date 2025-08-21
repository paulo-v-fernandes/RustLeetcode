pub struct Solution;


impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal: Vec<Vec<i32>> = vec![];
        let mut aux: Vec<i32> = vec![];

        for i in 0..num_rows {
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

            
            pascal.push(row.clone());
            aux = row.clone();
        }

        pascal
    }
}


fn main() {
    let rows: i32 = 5;
    let result = Solution::generate(rows);
    println!("{:?}", result);
}
