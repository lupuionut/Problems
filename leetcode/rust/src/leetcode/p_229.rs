// 229. Majority Element II
// ------------------------
// Boyer - Moore Voting Algorithm

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut first = None;
        let mut second = None;
        let mut first_count = 0;
        let mut second_count = 0;
        let mut ans = vec![];

        nums.iter().for_each(|n| {
            if Some(n) == first {
                first_count += 1;
            } else if Some(n) == second {
                second_count += 1;
            } else if first_count == 0 {
                first = Some(n);
                first_count += 1;
            } else if second_count == 0 {
                second = Some(n);
                second_count += 1;
            } else {
                first_count -= 1;
                second_count -= 1;
            }
        });

        first_count = 0;
        second_count = 0;
        nums.iter().for_each(|n| {
            if first == Some(n) {
                first_count += 1;
            }
            if second == Some(n) {
                second_count += 1;
            }
        });

        if first_count > nums.len() / 3 {
            ans.push(*first.unwrap());
        }

        if second_count > nums.len() / 3 {
            ans.push(*second.unwrap());
        }

        ans
    }
}
