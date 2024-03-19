// 2910. Minimum Number of Groups to Create a Valid Assignment
// -----------------------------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn min_groups_for_valid_assignment(balls: Vec<i32>) -> i32 {
        let mut freqs = HashMap::new();
        let mut ans = i32::MAX;

        balls.iter().for_each(|&n| {
            freqs.entry(n).and_modify(|count| *count += 1).or_insert(1);
        });

        let freqs = freqs.into_values().collect::<Vec<i32>>();
        let min_freq = *freqs.iter().min().unwrap();
        for guess_part_size in 1..min_freq + 1 {
            ans = ans.min(Solution::calculate(guess_part_size, &freqs));
        }

        ans
    }

    fn calculate(guess_part_size: i32, freqs: &Vec<i32>) -> i32 {
        let mut res = 0;
        for &freq in freqs.iter() {
            let num_groups = freq / guess_part_size;
            let remainder = freq % guess_part_size;
            if remainder > num_groups {
                return i32::MAX;
            }
            res += freq / (guess_part_size + 1);
            if freq % (guess_part_size + 1) != 0 {
                res += 1;
            }
        }

        res
    }
}
