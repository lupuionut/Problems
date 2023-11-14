// 303. Range Sum Query - Immutable
// --------------------------------

struct NumArray {
    ps: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut ps = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            ps[i + 1] = ps[i] + nums[i];
        }
        Self { ps }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.ps[(right + 1) as usize] - self.ps[left as usize]
    }
}
