// 2742. Painting the Walls
// ------------------------

impl Solution {
    pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
        let mut cache = vec![vec![-1; cost.len() + 1]; time.len() + 1];

        fn dp(
            i: usize,
            remain: i32,
            cost: &Vec<i32>,
            time: &Vec<i32>,
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if remain <= 0 {
                return 0;
            }
            if i == cost.len() {
                return i32::MAX;
            }
            if cache[i][remain as usize] != -1 {
                return cache[i][remain as usize];
            }

            let nxt = dp(i + 1, remain - 1 - time[i], cost, time, cache);
            let paint = if nxt == i32::MAX {
                i32::MAX
            } else {
                cost[i] + nxt
            };

            let skip = dp(i + 1, remain, cost, time, cache);
            cache[i][remain as usize] = paint.min(skip);
            cache[i][remain as usize]
        }

        dp(0, cost.len() as i32, &cost, &time, &mut cache)
    }
}
