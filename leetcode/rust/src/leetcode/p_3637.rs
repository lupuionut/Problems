// 3637. Trionic Array I
// ---------------------
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let desired = vec!['i', 'd', 'i'];
        let mut res = vec![];

        let mut last = ' ';
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] && last != 'i' {
                res.push('i');
                last = 'i';
                continue;
            }
            if nums[i] < nums[i - 1] && last != 'd' {
                res.push('d');
                last = 'd';
                continue;
            }
            if nums[i] == nums[i - 1] && last != 'e' {
                res.push('e');
                last = 'e';
                continue;
            }
        }

        res == desired
    }
}
