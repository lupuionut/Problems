// 2971. Find Polygon With the Largest Perimeter
// ---------------------------------------------

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i64 {
        nums.sort();
        let mut curr = nums[0] as i64 + nums[1] as i64;
        let mut best = -1;

        for i in 2..nums.len() {
            let n = nums[i] as i64;
            if curr > n {
                best = curr + n;
            }
            curr += n;
        }
        best
    }
}
