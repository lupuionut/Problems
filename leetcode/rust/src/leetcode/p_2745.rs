// 2745. Construct the Longest New String
// --------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn longest_string(x: i32, y: i32, z: i32) -> i32 {
        let mut cache: HashMap<(i32, i32, i32, char), i32> = HashMap::new();

        fn recurse(
            x: i32,
            y: i32,
            z: i32,
            prev: char,
            cache: &mut HashMap<(i32, i32, i32, char), i32>,
        ) -> i32 {
            if cache.contains_key(&(x, y, z, prev)) {
                return *cache.get(&(x, y, z, prev)).unwrap();
            }

            let mut best = 0;

            match prev {
                'x' => {
                    if y > 0 {
                        best = best.max(2 + recurse(x, y - 1, z, 'y', cache));
                    }
                }
                'y' | 'z' => {
                    if x > 0 {
                        best = best.max(2 + recurse(x - 1, y, z, 'x', cache));
                    }
                    if z > 0 {
                        best = best.max(2 + recurse(x, y, z - 1, 'z', cache));
                    }
                }
                _ => {
                    if x > 0 {
                        best = best.max(2 + recurse(x - 1, y, z, 'x', cache))
                    }
                    if y > 0 {
                        best = best.max(2 + recurse(x, y - 1, z, 'y', cache));
                    }
                    if z > 0 {
                        best = best.max(2 + recurse(x, y, z - 1, 'z', cache))
                    }
                }
            }

            cache.insert((x, y, z, prev), best);
            best
        }

        recurse(x, y, z, '0', &mut cache)
    }
}
