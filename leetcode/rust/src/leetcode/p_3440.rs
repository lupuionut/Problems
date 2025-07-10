// 3440. Reschedule Meetings for Maximum Free Time II
// --------------------------------------------------

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut events = vec![];
        let mut breaks = vec![];
        let mut curr = 0;
        for i in 0..start_time.len() {
            breaks.push(start_time[i] - curr);
            events.push(end_time[i] - start_time[i]);
            curr = end_time[i];
        }
        breaks.push(event_time - curr);

        let mut max_left = vec![0; breaks.len()];
        let mut max_right = vec![0; breaks.len()];

        curr = 0;
        for i in 0..breaks.len() {
            curr = curr.max(breaks[i]);
            max_left[i] = curr;
        }

        curr = 0;
        for i in (0..breaks.len()).rev() {
            curr = curr.max(breaks[i]);
            max_right[i] = curr;
        }

        for i in 0..events.len() {
            let s = breaks[i] + breaks[i + 1];
            if i > 0 {
                if max_left[i - 1] >= events[i] {
                    ans = ans.max(s + events[i]);
                    continue;
                }
            }
            if i < events.len() - 1 {
                if max_right[i + 2] >= events[i] {
                    ans = ans.max(s + events[i]);
                    continue;
                }
            }
            ans = ans.max(s);
        }

        ans
    }
}
