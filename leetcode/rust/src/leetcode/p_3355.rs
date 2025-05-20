// 3355. Zero Array Transformation I
// ---------------------------------
impl Solution {
    pub fn is_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> bool {
        let mut delta = vec![0; nums.len() + 1];
        for query in queries {
            let s = query[0];
            let e = query[1];
            delta[s as usize] += 1;
            delta[(e + 1) as usize] -= 1;
        }
        let mut d = 0;
        for i in 0..nums.len() {
            d += delta[i];
            if d < nums[i] {
                return false;
            }
        }

        true
    }
}
