// 386. Lexicographical Numbers
// ----------------------------

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn run(start: i32, max: i32, ans: &mut Vec<i32>) {
            if start <= max {
                ans.push(start);
            } else {
                return;
            }
            for i in 0..=9 {
                run(start * 10 + i, max, ans);
            }
        }

        let mut ans = vec![];
        for i in 1..=9 {
            run(i, n, &mut ans);
        }
        ans
    }
}
