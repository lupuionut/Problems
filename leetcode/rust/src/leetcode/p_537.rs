// 537. Complex Number Multiplication

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let parts1: Vec<&str> = num1.split('+').collect();
        let parts2: Vec<&str> = num2.split('+').collect();
        let mut real: i32 = parts1[0].parse::<i32>().unwrap() * parts2[0].parse::<i32>().unwrap();
        let imaginary: i32 = (parts1[0].parse::<i32>().unwrap()
            * parts2[1].trim_matches(|c| c == 'i').parse::<i32>().unwrap())
            + (parts1[1].trim_matches(|c| c == 'i').parse::<i32>().unwrap()
                * parts2[0].parse::<i32>().unwrap());
        real += parts1[1].trim_matches(|c| c == 'i').parse::<i32>().unwrap()
            * parts2[1].trim_matches(|c| c == 'i').parse::<i32>().unwrap()
            * -1;
        format!("{}+{}i", real, imaginary)
    }
}
