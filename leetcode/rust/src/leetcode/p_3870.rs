// 3870. Count Commas in Range
// ---------------------------
impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        let mut count = 0;
        let mut m = 1000;
        while (m + 1000) < n {
            count += 1000;
            m += 1000;
        }
        for i in m..=n {
            if i >= 1000 {
                count += 1;
            }
        }

        count
    }
}
