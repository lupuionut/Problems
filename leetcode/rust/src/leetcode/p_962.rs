// 962. Maximum Width Ramp
// -----------------------

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut possible = vec![(-1, i32::MAX)];
        let mut best = 0;

        nums.iter().enumerate().for_each(|(k, &v)| {
            let mut left = 0;
            let mut right = possible.len() - 1;

            while left < right {
                let mid = (left + right) / 2;
                if possible[mid].1 <= v {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }

            if possible[left].1 <= v {
                best = best.max(k as i32 - possible[left].0);
            }

            let last = *possible.last().unwrap();
            if v < last.1 {
                possible.push((k as i32, v));
            }
        });

        best
    }
}
