// 1342. Number of Steps to Reduce a Number to Zero
// ------------------------------------------------

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut counter: i32 = 0;
        let binary = format!("{:b}", num);
        counter += binary.len() as i32;

        for c in binary.chars() {
            if c == '1' {
                counter += 1;
            }
        }

        return (counter - 1);
    }
}
