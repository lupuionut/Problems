// 3024. Type of Triangle
// ----------------------

impl Solution {
    pub fn triangle_type(nums: Vec<i32>) -> String {
        if nums[0] == nums[1] && nums[1] == nums[2] {
            return "equilateral".to_string();
        }
        if nums[0] + nums[1] > nums[2] && nums[0] + nums[2] > nums[1] && nums[1] + nums[2] > nums[0]
        {
            if nums[0] == nums[1] || nums[1] == nums[2] || nums[0] == nums[2] {
                return "isosceles".to_string();
            }
            return "scalene".to_string();
        }

        "none".to_string()
    }
}
