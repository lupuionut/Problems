// 3439. Reschedule Meetings for Maximum Free Time I
// -------------------------------------------------

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut spaces = vec![];
        let mut curr = 0;
        for i in 0..start_time.len() {
            spaces.push(start_time[i] - curr);
            curr = end_time[i];
        }
        if curr < event_time {
            spaces.push(event_time - curr);
        }

        let k = k as usize;
        let mut curr = 0;
        for i in 0..spaces.len() {
            curr += spaces[i];
            ans = ans.max(curr);
            if i >= k {
                curr -= spaces[i - k];
            }
        }

        ans
    }
}
