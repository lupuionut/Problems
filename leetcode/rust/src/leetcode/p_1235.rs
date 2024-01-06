// 1235. Maximum Profit in Job Scheduling
// --------------------------------------

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut intervals = vec![];
        let mut s_it = start_time.iter();
        let mut e_it = end_time.iter();
        let mut it = (s_it.zip(e_it)).zip(profit.iter());

        for ((s, e), p) in it {
            intervals.push((*s, *e, *p));
        }
        intervals.sort();

        fn dp(i: usize, intvals: &Vec<(i32, i32, i32)>, cache: &mut Vec<i32>) -> i32 {
            if i >= intvals.len() {
                return 0;
            }

            if cache[i] != -1 {
                return cache[i];
            }

            // dont take profit
            let mut best = dp(i + 1, intvals, cache);

            // take and move to next intval with start >= current end time
            let next_index = intvals.partition_point(|intv| intv.0 < intvals[i].1);
            best = best.max(intvals[i].2 + dp(next_index, intvals, cache));
            cache[i] = best;
            cache[i]
        }

        let mut cache = vec![-1; intervals.len()];
        dp(0, &intervals, &mut cache)
    }
}
