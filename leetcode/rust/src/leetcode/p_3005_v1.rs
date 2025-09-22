// 3005. Count Elements With Maximum Frequency
// -------------------------------------------

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut maxf = 0;
        let mut freqs = vec![0; 101];
        for num in nums {
            freqs[num as usize] += 1;
            if freqs[num as usize] > maxf {
                maxf = freqs[num as usize];
                count = 1;
            } else if freqs[num as usize] == maxf {
                count += 1;
            }
        }

        count * maxf
    }
}
