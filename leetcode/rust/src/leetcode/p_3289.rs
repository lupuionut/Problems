// 3289. The Two Sneaky Numbers of Digitville
// ------------------------------------------
impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; nums.len()];
        let mut ans = vec![];
        for num in nums {
            if seen[num as usize] == true {
                ans.push(num);
            } else {
                seen[num as usize] = true;
            }
        }
        ans
    }
}
