// 744. Find Smallest Letter Greater Than Target
// ---------------------------------------------

impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        match letters.iter().find(|&c| c.gt(&target)) {
            Some(a) => *a,
            None => letters[0],
        }
    }
}
