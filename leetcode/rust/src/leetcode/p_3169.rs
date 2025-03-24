// 3169. Count Days Without Meetings
// ---------------------------------

impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort();
        let mut ans = 0;
        let mut end = 1;

        for meeting in meetings {
            let s = meeting[0];
            let e = meeting[1] + 1;
            if s > days {
                break;
            }

            if s > end {
                ans += (s - end);
            }
            end = end.max(e);
        }

        if end <= days {
            ans += (days - end + 1);
        }

        ans
    }
}
