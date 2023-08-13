// 2369. Check if There is a Valid Partition For The Array
// -------------------------------------------------------

#[derive(Clone, Eq, PartialEq)]
enum Cache {
    T,
    F,
    N,
}

impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {
        let mut cache: Vec<Cache> = vec![Cache::N; nums.len()];

        fn dfs(i: usize, nums: &Vec<i32>, cache: &mut Vec<Cache>) -> bool {
            if i == nums.len() {
                return true;
            }

            match cache[i] {
                Cache::T => return true,
                Cache::F => return false,
                Cache::N => {}
            }

            let mut ans = false;
            if i < nums.len() - 1 {
                if nums[i] == nums[i + 1] {
                    ans |= dfs(i + 2, nums, cache);
                }
            }
            if i < nums.len() - 2 {
                if (nums[i] == nums[i + 1] && nums[i + 1] == nums[i + 2])
                    || (nums[i] == nums[i + 1] - 1 && nums[i + 1] == nums[i + 2] - 1)
                {
                    ans |= dfs(i + 3, nums, cache);
                }
            }

            if ans == true {
                cache[i] = Cache::T;
            } else {
                cache[i] = Cache::F;
            }
            ans
        }

        dfs(0, &nums, &mut cache)
    }
}
