// 2770. Maximum Number of Jumps to Reach the Last Index
// -----------------------------------------------------
impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        fn jump(i: usize, nums: &Vec<i32>, target: i32, cache: &mut Vec<Option<i32>>) -> i32 {
            if i == nums.len() - 1 {
                return 0;
            }
            if let Some(v) = cache[i] {
                return v;
            }

            let mut best = i32::MIN;

            for j in i + 1..nums.len() {
                if (nums[j] - nums[i]).abs() <= target {
                    let next = jump(j, nums, target, cache);
                    if next != -1 {
                        best = best.max(1 + next);
                    }
                }
            }
            if best == i32::MIN {
                best = -1;
            }
            cache[i] = Some(best);
            best
        }

        let mut cache = vec![None; nums.len()];
        jump(0, &nums, target, &mut cache)
    }
}
