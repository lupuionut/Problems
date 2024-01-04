// 2870. Minimum Number of Operations to Make Array Empty
// ------------------------------------------------------

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut freq = vec![0; 1_000_001];
        nums.iter().for_each(|&n| {
            freq[n as usize] += 1;
        });

        let mut ans = 0;
        for i in 0..1_000_001 {
            let mut val = freq[i];
            if val == 0 {
                continue;
            }
            if val == 1 {
                return -1;
            }

            if val % 3 == 0 {
                ans += val / 3;
            } else {
                while val > 1 {
                    if val % 3 == 0 {
                        ans += val / 3;
                        val = 0;
                        break;
                    }
                    val -= 2;
                    ans += 1;
                }
                if val == 1 {
                    return -1;
                }
            }
        }

        ans
    }
}
