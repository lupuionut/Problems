// 2024. Maximize the Confusion of an Exam
// ---------------------------------------

impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut best = 0;
        let answer: Vec<char> = answer_key.chars().collect();

        for key in ['T', 'F'] {
            let mut left = 0;
            let mut right = 0;
            let mut counter = 0;

            while right < answer_key.len() {
                if answer[right as usize] != key {
                    counter += 1;
                }
                while counter > k {
                    if answer[left as usize] != key {
                        counter -= 1;
                    }
                    left += 1;
                }
                best = best.max(right - left + 1);
                right += 1;
            }
        }
        best as i32
    }
}
