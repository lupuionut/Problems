// 2389. Longest Subsequence With Limited Sum
// ------------------------------------------

impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut nums = nums;
        let mut ps = vec![0; nums.len() + 1];
        nums.sort();

        for i in 0..nums.len() {
            ps[i + 1] = ps[i] + nums[i];
        }

        for query in queries {
            match ps.binary_search(&query) {
                Ok(key) => {
                    ans.push(key as i32);
                }
                Err(mut key) => {
                    if key != 0 {
                        key -= 1;
                    };
                    ans.push(key as i32);
                }
            };
        }

        ans
    }
}
