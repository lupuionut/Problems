// 2966. Divide Array Into Arrays With Max Difference
// --------------------------------------------------

impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        nums.sort();

        let mut diff = 0;
        let mut curr = vec![0; 3];
        let mut is_possible = true;

        for i in 0..nums.len() {
            let key = i % 3;
            if key == 0 {
                if i > 0 {
                    ans.push(curr);
                }
                diff = 0;
                curr = vec![0; 3];
            } else {
                diff += (nums[i] - curr[key - 1]);
                if diff > k {
                    is_possible = false;
                    break;
                }
            }
            curr[key] = nums[i];
        }

        if !is_possible {
            return vec![];
        }

        ans.push(curr);
        ans
    }
}
