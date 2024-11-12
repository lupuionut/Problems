// 2070. Most Beautiful Item for Each Query
// ----------------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut best: HashMap<i32, i32> = HashMap::new();
        let mut ans = vec![];
        let mut price_beauty = vec![];
        items.iter().for_each(|item| {
            let price = item[0];
            let beauty = item[1];
            if let Some(&curr) = best.get(&price) {
                best.insert(price, curr.max(beauty));
            } else {
                best.insert(price, beauty);
            }
        });
        for (price, beauty) in best {
            price_beauty.push((price, beauty));
        }
        price_beauty.sort();
        for i in 1..price_beauty.len() {
            price_beauty[i].1 = price_beauty[i - 1].1.max(price_beauty[i].1);
        }

        for i in 0..queries.len() {
            let max_price = queries[i];
            let mut left = 0;
            let mut right = (price_beauty.len() - 1) as i32;
            let mut best = 0;

            while left <= right {
                let middle = (left + right) / 2;

                if price_beauty[middle as usize].0 == max_price {
                    best = price_beauty[middle as usize].1;
                    break;
                }
                if price_beauty[middle as usize].0 > max_price {
                    right = middle - 1;
                } else {
                    best = best.max(price_beauty[middle as usize].1);
                    left = middle + 1;
                }
            }
            ans.push(best);
        }

        ans
    }
}
