// 57. Insert Interval
// -------------------

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval = new_interval;
        let mut inserted = false;
        let mut ans = vec![];

        intervals.into_iter().for_each(|interval| {
            // if it ends before start of insert
            if interval[1] < new_interval[0] {
                ans.push(interval);
            // or it starts after the end of insert
            } else if interval[0] > new_interval[1] {
                if !inserted {
                    ans.push(new_interval.clone());
                    inserted = true;
                }
                ans.push(interval);
            } else {
                new_interval[0] = new_interval[0].min(interval[0]);
                new_interval[1] = new_interval[1].max(interval[1]);
            }
        });

        if !inserted {
            ans.push(new_interval);
        }

        ans
    }
}
