// 1493. Longest Subarray of 1's After Deleting One Element
// --------------------------------------------------------

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut sub: Vec<i32> = vec![];
        let mut mark = 0;
        let mut longest = 0;
        let n = nums.len();

        for i in 0..=n {
            if i == n {
                sub.push((i - mark) as i32);
                break;
            }
            if nums[i] == 0 {
                sub.push((i - mark) as i32);
                mark = i + 1;
            }
        }

        if sub.len() == 1 {
            longest = sub[0] - 1;
        } else {
            for i in 0..sub.len() - 1 {
                longest = longest.max(sub[i] + sub[i + 1]);
            }
        }

        longest
    }
}
