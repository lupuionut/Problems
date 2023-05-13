//    2169. Count Operations to Obtain Zero
//    -------------------------------------

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        if num1 == 0 || num2 == 0 {
            return 0;
        }
        if num1 > num2 {
            return 1 + Solution::count_operations(num1 - num2, num2);
        } else {
            return 1 + Solution::count_operations(num1, num2 - num1);
        }
    }
}
