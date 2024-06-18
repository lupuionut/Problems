// 826. Most Profit Assigning Work
// -------------------------------

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        worker.sort();
        let mut i0 = difficulty.iter();
        let mut i1 = i0.zip(profit.iter());
        let mut zipped = vec![];
        i1.for_each(|p| zipped.push(p));
        zipped.sort();

        let mut i = 0;
        let mut best = 0;
        let mut ans = 0;

        for (&diff, &prof) in zipped {
            while i < worker.len() && diff > worker[i] {
                ans += best;
                i += 1;
            }

            best = best.max(prof);
        }

        if i < worker.len() {
            ans += (worker.len() - i) as i32 * best;
        }

        ans
    }
}
