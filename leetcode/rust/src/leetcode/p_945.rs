// 945. Minimum Increment to Make Array Unique
// -------------------------------------------

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut curr = -1;
        nums.iter().for_each(|&n| {
            if n > curr {
                curr = n;
            } else if n == curr {
                curr += 1;
                ans += 1;
            } else {
                curr += 1;
                ans += (curr - n);
            }
        });

        ans
    }
}
