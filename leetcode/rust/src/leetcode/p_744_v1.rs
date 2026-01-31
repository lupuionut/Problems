// 744. Find Smallest Letter Greater Than Target
// ---------------------------------------------
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut l = 0;
        let mut r = letters.len();
        while l < r {
            let mid = (l + r) / 2;
            if letters[mid] <= target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        letters[l % letters.len()]
    }
}
