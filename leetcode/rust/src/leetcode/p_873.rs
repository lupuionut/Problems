// 873. Length of Longest Fibonacci Subsequence
// --------------------------------------------

use std::collections::HashSet;

impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let max = arr[n - 1];
        let mut values = HashSet::new();

        arr.iter().for_each(|&x| {
            values.insert(x);
        });

        fn fib(a: i32, b: i32, len: i32, max: i32, values: &HashSet<i32>) -> i32 {
            let c = a + b;
            if c > max || !values.contains(&c) {
                return len;
            }
            return fib(b, c as i32, len + 1, max, values);
        }

        let mut best = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let a = arr[i];
                let b = arr[j];
                best = best.max(fib(a, b, 2, max, &values));
            }
        }

        if best < 3 {
            return 0;
        }
        best
    }
}
