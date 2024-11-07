// 2275. Largest Combination With Bitwise AND Greater Than Zero
// ------------------------------------------------------------

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut bits = vec![0; 32];
        candidates.into_iter().for_each(|mut n| {
            let mut i = 0;
            while n > 0 {
                bits[i] += (n % 2);
                n /= 2;
                i += 1;
            }
        });
        *bits.iter().max().unwrap()
    }
}
