// 1575. Count All Possible Routes
// -------------------------------

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let mut cache: Vec<Vec<i64>> = vec![vec![-1; (fuel + 1) as usize]; locations.len()];
        fn dfs(
            locations: &Vec<i32>,
            start: i32,
            finish: i32,
            fuel: i32,
            cache: &mut Vec<Vec<i64>>,
        ) -> i64 {
            if fuel < 0 {
                return 0;
            }

            if cache[start as usize][fuel as usize] != -1 {
                return cache[start as usize][fuel as usize];
            }
            let mut total = 0;

            if start == finish {
                total += 1;
            }

            for i in 0..locations.len() {
                if i as i32 == start {
                    continue;
                }
                let cost = (locations[i] - locations[start as usize]).abs();
                total += dfs(locations, i as i32, finish, fuel - cost, cache) % 1_000_000_007;
            }
            cache[start as usize][fuel as usize] = total;
            total
        }

        (dfs(&locations, start, finish, fuel, &mut cache) % 1_000_000_007) as i32
    }
}
