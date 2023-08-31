// 1326. Minimum Number of Taps to Open to Water a Garden
// ------------------------------------------------------

impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut max_ranges = vec![0; (n + 1) as usize];
        let mut end = 0;
        let mut taps = 0;
        let mut farthest = 0;
        let mut i = 0;

        // calculate for each range index, the max range that covers [left..right]
        ranges.iter().enumerate().for_each(|(k, &v)| {
            let left = 0.max(k as i32 - v);
            let right = n.min(k as i32 + v);
            max_ranges[left as usize] = left.max(right);
        });

        // while we are not covering the whole n length we take the farthest interval
        while end < n {
            // loop from last position until the current end position to find the next end (farthest)
            while i <= end {
                farthest = farthest.max(max_ranges[i as usize]);
                i += 1;
            }

            // if the next farthest (end) is smaller or equal with current end there is no way to advance
            if farthest <= end {
                return -1;
            }

            end = farthest;
            taps += 1;
        }

        taps
    }
}
