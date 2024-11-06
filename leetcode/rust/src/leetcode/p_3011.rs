// 3011. Find if Array Can Be Sorted
// ---------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn can_sort_array(mut nums: Vec<i32>) -> bool {
        let mut bin = HashMap::new();

        fn count_set_bits(n: i32) -> i32 {
            let s = format!("{:b}", n);
            let mut counter = 0;
            s.chars().for_each(|c| {
                if c == '1' {
                    counter += 1;
                }
            });
            counter
        }

        nums.iter().for_each(|&n| {
            bin.insert(n, count_set_bits(n));
        });

        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] > nums[j] {
                    let count_i = bin.get(&nums[i]).unwrap();
                    let count_j = bin.get(&nums[j]).unwrap();
                    if count_i != count_j {
                        return false;
                    }
                    let t = nums[i];
                    nums[i] = nums[j];
                    nums[j] = t;
                }
            }
        }

        true
    }
}
