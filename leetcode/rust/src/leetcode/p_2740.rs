// 2740. Find the Value of the Partition
// -------------------------------------

impl Solution {
    pub fn find_value_of_partition(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut ans = 1_000_000_000;
        let mut iter = nums.windows(2);
        while let Some(&[a, b]) = iter.next() {
            ans = ans.min(b - a);
        }

        ans
    }
}
