// 3718. Smallest Missing Multiple of K
// ------------------------------------
impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut existing = vec![false; nums.len()];
        for &num in &nums {
            if num % k == 0 {
                let j = (num / k) as usize - 1;
                if j < nums.len() {
                    existing[j] = true;
                }
            }
        }
        for i in 0..existing.len() {
            if existing[i] == false {
                return (i + 1) as i32 * k;
            }
        }
        (nums.len() as i32 + 1) * k 
    }
}
