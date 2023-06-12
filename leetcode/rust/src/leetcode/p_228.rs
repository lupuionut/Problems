// 228. Summary Ranges
// -------------------

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut left = 0;
        let mut right = 0;
        let mut ans: Vec<String> = Vec::new();

        for i in 1..nums.len() {
            if nums[right] + 1 != nums[i] {
                if left == right {
                    ans.push(nums[left].to_string());
                } else {
                    let val = format!(
                        "{}->{}",
                        nums[left].to_string().as_str(),
                        nums[right].to_string().as_str()
                    );
                    ans.push(val);
                }
                left = i;
                right = i;
            } else {
                right = i;
            }
        }

        if left == right {
            if left < nums.len() {
                ans.push(nums[left].to_string());
            }
        } else {
            let val = format!(
                "{}->{}",
                nums[left].to_string().as_str(),
                nums[right].to_string().as_str()
            );
            ans.push(val);
        }

        ans
    }
}
