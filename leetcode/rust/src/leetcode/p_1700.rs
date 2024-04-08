// 1700. Number of Students Unable to Eat Lunch
// --------------------------------------------

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut ones = 0;
        let mut zeros = 0;

        students.iter().for_each(|&n| {
            if n == 0 {
                zeros += 1;
            } else {
                ones += 1;
            }
        });

        for i in 0..sandwiches.len() {
            if sandwiches[i] == 0 && zeros > 0 {
                zeros -= 1;
            } else if sandwiches[i] == 1 && ones > 0 {
                ones -= 1;
            } else {
                return (sandwiches.len() - i) as i32;
            }
        }

        0
    }
}
