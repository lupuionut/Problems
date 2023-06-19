// 2672. Number of Adjacent Elements With the Same Color
// -----------------------------------------------------

impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut nums = vec![0; n as usize];
        let mut ans = vec![0; queries.len()];
        let mut current = 0;

        for i in 0..queries.len() {
            let idx = queries[i][0] as usize;
            let color = queries[i][1];

            if idx > 0 {
                if nums[idx - 1] == nums[idx] && nums[idx] != 0 {
                    current -= 1;
                }
                if nums[idx - 1] == color {
                    current += 1;
                }
            }
            if idx < (n - 1) as usize {
                if nums[idx + 1] == nums[idx] && nums[idx] != 0 {
                    current -= 1;
                }
                if nums[idx + 1] == color {
                    current += 1;
                }
            }
            nums[idx] = color;
            ans[i] = current;
        }

        ans
    }
}
