// 1752. Check if Array Is Sorted and Rotated
// ------------------------------------------

impl Solution {
    pub fn check(mut nums: Vec<i32>) -> bool {
        let mut original = nums.clone();
        nums.sort();

        fn shift(original: &Vec<i32>, by: usize) -> Vec<i32> {
            let n = original.len();
            let mut ans = vec![0; n];
            for i in 0..n {
                ans[i] = original[(i + by) % n];
            }
            ans
        }

        for i in 0..nums.len() {
            let curr = shift(&original, i);
            if curr == nums {
                return true;
            }
        }

        false
    }
}
