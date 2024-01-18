// 70. Climbing Stairs
// -------------------

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        fn climb(curr: i32, n: i32, cache: &mut Vec<i32>) -> i32 {
            if curr > n {
                return 0;
            }

            if curr == n {
                return 1;
            }

            if cache[curr as usize] != -1 {
                return cache[curr as usize];
            }

            let mut sum = 0;
            sum += climb(curr + 1, n, cache);
            sum += climb(curr + 2, n, cache);
            cache[curr as usize] = sum;
            sum
        }

        let mut cache = vec![-1; (n + 1) as usize];
        climb(0, n, &mut cache)
    }
}
