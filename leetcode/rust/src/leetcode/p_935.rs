// 935. Knight Dialer
// ------------------

impl Solution {
    pub fn knight_dialer(n: i32) -> i32 {
        let moves: &[Vec<usize>] = &[
            vec![4, 6],
            vec![6, 8],
            vec![7, 9],
            vec![4, 8],
            vec![0, 3, 9],
            vec![],
            vec![0, 1, 7],
            vec![2, 6],
            vec![1, 3],
            vec![2, 4],
        ];

        fn dp(
            i: usize,
            size: i32,
            max: i32,
            moves: &[Vec<usize>],
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if size == max {
                return 1;
            }

            let in_cache = cache[i][size as usize];
            if in_cache != -1 {
                return in_cache;
            }

            let mut ans = 0;

            moves[i].iter().for_each(|&next| {
                ans += dp(next, size + 1, max, moves, cache) % 1_000_000_007;
                ans %= 1_000_000_007;
            });
            cache[i][size as usize] = ans;
            ans
        }

        let mut ans = 0;
        let mut cache = vec![vec![-1; n as usize]; 10];
        for i in 0..10 {
            ans += dp(i, 1, n, &moves, &mut cache) % 1_000_000_007;
            ans %= 1_000_000_007;
        }

        ans
    }
}
