// 42. Trapping Rain Water
// -----------------------

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut right_max = vec![0; height.len()];
        let mut left_max = 0;

        for i in (0..height.len() - 1).rev() {
            right_max[i] = right_max[i + 1].max(height[i + 1]);
        }
        for i in 1..height.len() {
            left_max = left_max.max(height[i - 1]);
            let water = left_max.min(right_max[i]);
            ans += 0.max(water - height[i]);
        }
        ans
    }
}
