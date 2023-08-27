// 403. Frog Jump
// --------------

use std::collections::HashMap;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let mut cache = HashMap::new();

        fn jump_from(
            k: i32,
            i: usize,
            stones: &Vec<i32>,
            cache: &mut HashMap<(i32, usize), bool>,
        ) -> bool {
            let n = stones.len() - 1;
            let ds = &[k - 1, k, k + 1];
            let mut ans = false;

            if let Some(&value) = cache.get(&(k, i)) {
                return value;
            }

            for &d in ds {
                if d > 0 {
                    let possible = stones[i] + d;
                    match stones.binary_search(&possible) {
                        Ok(new_stone) => {
                            if new_stone == n {
                                ans = true;
                                break;
                            } else {
                                ans = ans | jump_from(d, new_stone, stones, cache);
                            }
                        }
                        Err(_) => {}
                    }
                }
            }

            cache.insert((k, i), ans);
            ans
        }

        jump_from(0, 0, &stones, &mut cache)
    }
}
