// 779. K-th Symbol in Grammar
// ---------------------------

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let n = n - 1;
        let k = k - 1;

        fn recurse(n: i32, k: i32) -> i32 {
            if n == 0 {
                return 0;
            }
            let parent = recurse(n - 1, k / 2);
            match (parent, k & 1) {
                (0, 0) => return 0,
                (0, 1) => return 1,
                (1, 0) => return 1,
                (1, 1) => return 0,
                _ => return -1,
            }
        }

        recurse(n, k)
    }
}
