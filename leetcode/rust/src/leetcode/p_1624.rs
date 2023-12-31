// 1624. Largest Substring Between Two Equal Characters
// ----------------------------------------------------

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first_occurence = vec![-1; 26];
        let mut longest = -1;

        s.chars().enumerate().for_each(|(i, c)| {
            let key = (c as usize) - 97;
            let curr = i as i32;
            if first_occurence[key] != -1 {
                longest = longest.max(curr - first_occurence[key] - 1);
            } else {
                first_occurence[key] = curr;
            }
        });

        longest
    }
}
