// 1963. Minimum Number of Swaps to Make the String Balanced
// ---------------------------------------------------------

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut opens = 0;
        let mut swaps = 0;

        s.chars().for_each(|c| {
            if c == '[' {
                opens += 1;
            } else {
                if opens > 0 {
                    opens -= 1;
                } else {
                    opens += 1;
                    swaps += 1;
                }
            }
        });

        swaps
    }
}
