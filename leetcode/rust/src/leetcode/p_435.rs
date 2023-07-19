// 435. Non-overlapping Intervals
// ------------------------------

impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        let mut ans = 0;
        intervals.sort();
        let mut last = intervals[0][0];

        intervals.iter().for_each(|x| {
            if x[0] < last {
                last = last.min(x[1]);
                ans += 1;
            } else {
                last = x[1];
            }
        });

        ans
    }
}
