// 1406. Stone Game III
// --------------------

use core::i32::MIN;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut prefix_sum: Vec<i32> = vec![0; stone_value.len() + 1];
        let mut cache: Vec<i32> = vec![-1; stone_value.len() + 1];

        for i in 1..prefix_sum.len() {
            prefix_sum[i] = prefix_sum[i - 1] + stone_value[i - 1];
        }

        fn get_delta(idx: usize, prefix_sum: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if idx == prefix_sum.len() - 1 {
                return 0;
            }

            if cache[idx] != -1 {
                return cache[idx];
            }

            let mut best = MIN;
            let mut next = idx;
            for x in 1..=3 {
                next = if (idx + x) >= prefix_sum.len() {
                    prefix_sum.len() - 1
                } else {
                    idx + x
                };
                let my_stones = prefix_sum[next] - prefix_sum[idx];
                let his_stones = get_delta(next, prefix_sum, cache);
                if (my_stones - his_stones) > best {
                    best = my_stones - his_stones;
                }
            }
            cache[idx] = best;
            best
        }

        let delta = get_delta(0, &prefix_sum, &mut cache);
        let total: i32 = prefix_sum[prefix_sum.len() - 1];
        let bob = (total - delta) / 2;
        let alice = total - bob;
        if alice > bob {
            return "Alice".to_string();
        }
        if alice < bob {
            return "Bob".to_string();
        }
        "Tie".to_string()
    }
}
