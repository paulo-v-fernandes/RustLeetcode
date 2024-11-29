pub fn roman_to_int(s: String) -> i32 {
    let mut total = 0;
    let mut prev_value = 0;

    for ch in s.chars().rev() {
        let value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
        println!("{}", value);
        if prev_value > value{
            total -= value;
        }
        else{
            total += value;
        }    
        prev_value = value;
    }
    total

}

fn main() {
    let s1 = "II".to_string();
    println!("Result: {}", roman_to_int(s1)); // Output: 3

}
