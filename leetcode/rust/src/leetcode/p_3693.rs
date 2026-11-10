// 3693. Climbing Stairs II
// ------------------------
 
impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        fn dp(curr: i32, n: i32, costs: &Vec<i32>, cache: &mut Vec<i32>) -> i32 {
            if curr == n {
                return 0;
            }
            if cache[curr as usize] != -1 {
                return cache[curr as usize];
            }
            let mut best = i32::MAX;
            for i in 1..=3 {
                let j = curr + i;
                if j > n {
                    break;
                }
                best = best.min(costs[(j-1) as usize] + (i*i) + dp(j, n, costs, cache));
            }
            cache[curr as usize] = best;
            best
        }
        let mut cache = vec![-1; n as usize];
        dp(0, n, &costs, &mut cache)
    }
}
