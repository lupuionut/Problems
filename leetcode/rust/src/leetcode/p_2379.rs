// 2379. Minimum Recolors to Get K Consecutive Black Blocks
// --------------------------------------------------------

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let mut blacks = 0;
        let mut l = 0;
        let mut chars = blocks.chars().collect::<Vec<char>>();
        let mut ans = i32::MAX;

        for r in 0..chars.len() {
            if chars[r] == 'B' {
                blacks += 1;
            }
            while (r - l + 1) as i32 > k {
                if chars[l] == 'B' {
                    blacks -= 1;
                }
                l += 1;
            }
            ans = ans.min(k - blacks);
        }
        ans
    }
}
