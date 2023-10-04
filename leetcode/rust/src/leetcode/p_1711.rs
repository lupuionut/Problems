// 1711. Count Good Meals
// ----------------------

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        const modulo: i32 = 1e9 as i32 + 7;
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;

        deliciousness.iter().for_each(|x| {
            for i in 0..22 {
                let target = (1 << i) - x;
                if counter.contains_key(&target) {
                    ans += counter.get(&target).unwrap();
                    ans = ans % modulo;
                }
            }
            counter.entry(*x).and_modify(|c| *c += 1).or_insert(1);
        });

        ans % modulo
    }
}
