// 1288. Remove Covered Intervals
// ------------------------------
impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        intervals.sort();
        let mut curr: Option<&Vec<i32>> = None;
        for ival in &intervals {
            if let Some(c) = curr {
                if ival[1] <= c[1] {
                    continue;
                }
                if ival[0] == c[0] && ival[1] > c[1] {
                    curr = Some(ival);
                    continue;
                }   
                curr = Some(ival);
                ans += 1;
            } else {
                curr = Some(ival);
                ans += 1;
            }  
        }
        ans
    }
}
