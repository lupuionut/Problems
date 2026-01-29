// 2976. Minimum Cost to Convert String I
// --------------------------------------
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let mut ans = 0i64;
        let mut dist = vec![vec![None; 26]; 26];

        for i in 0..original.len() {
            let x = original[i] as usize - 97;
            let y = changed[i] as usize - 97;
            if dist[x][y].is_none() {
                dist[x][y] = Some(cost[i]);
            } else {
                dist[x][y] = dist[x][y].min(Some(cost[i]));
            }
        }

        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if let Some(a) = dist[i][k] {
                        if let Some(b) = dist[k][j] {
                            if let Some(c) = dist[i][j] {
                                dist[i][j] = Some(c.min(a + b));
                            } else {
                                dist[i][j] = Some(a + b);
                            }
                        }
                    }
                }
            }
        }

        let mut a = target.chars();
        let mut it = source.chars().zip(a);

        for (x, y) in it {
            if x != y {
                let x = x as usize - 97;
                let y = y as usize - 97;
                if let Some(v) = dist[x][y] {
                    ans += v as i64;
                } else {
                    return -1;
                }
            }
        }

        ans
    }
}
