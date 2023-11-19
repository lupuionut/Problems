// 1887. Reduction Operations to Make the Array Elements Equal
// -----------------------------------------------------------

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut freq = vec![0; 50_001];
        let mut nums = nums;
        nums.sort();

        nums.iter().for_each(|&n| {
            freq[n as usize] += 1;
        });

        let n = nums.len();
        let mut acc = 0;

        if nums[0] != nums[n - 1] {
            for i in (0..n).rev() {
                if nums[i] == nums[0] {
                    break;
                }

                let number = nums[i] as usize;
                // visited
                if freq[number] == -1 {
                    continue;
                }
                ans += freq[number] + acc;
                acc += freq[number];
                // mark as visited
                freq[number] = -1;
            }
        }

        ans
    }
}
