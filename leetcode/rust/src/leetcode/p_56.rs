// 56. Merge Intervals
// -------------------

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut intervals = intervals;
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        intervals.iter().for_each(|interval| match ans.last() {
            Some(intval) => {
                if interval[0] <= intval[1] {
                    let n = if intval[1] > interval[1] {
                        vec![intval[0], intval[1]]
                    } else {
                        vec![intval[0], interval[1]]
                    };
                    ans.pop();
                    ans.push(n);
                } else {
                    ans.push(interval.to_vec());
                }
            }
            None => {
                ans.push(interval.to_vec());
            }
        });

        ans
    }
}
