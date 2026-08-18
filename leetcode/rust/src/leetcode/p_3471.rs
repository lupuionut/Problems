// 3471. Find the Largest Almost Missing Integer
// ---------------------------------------------
impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut visited = [0; 51];
        let n = nums.len() - k as usize;
        for i in 0..=n {
            let mut thispass = [false; 51];
            for j in i..i+k as usize {
                thispass[nums[j] as usize] = true;
            } 
            for j in 0..51 {
                if thispass[j] == true {
                    visited[j] += 1;
                }
            }
        }

        let mut ans = -1;
        for i in 0..51 {
            if visited[i] == 1 {
                ans = ans.max(i as i32);
            }
        }
        ans
    }
}
