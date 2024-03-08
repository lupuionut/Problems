// 3005. Count Elements With Maximum Frequency
// -------------------------------------------

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut freqs = vec![0; 101];
        let mut best = 0;
        let mut counter = 0;
        nums.iter().for_each(|&n| {
            freqs[n as usize] += 1;
            best = best.max(freqs[n as usize]);
        });
        freqs.iter().for_each(|&freq| {
            if freq == best {
                counter += 1;
            }
        });

        counter * best
    }
}
