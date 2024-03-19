// 621. Task Scheduler
// -------------------

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut counts = vec![0; 26];
        tasks.iter().for_each(|&n| {
            let key = n as usize - 65;
            counts[key] += 1;
        });
        counts.sort_by(|a, b| b.cmp(&a));
        let group_count = counts[0] - 1;
        let mut items = group_count * n;
        for i in 1..26 {
            items -= counts[i].min(group_count);
        }
        let cooling = 0.max(items);
        tasks.len() as i32 + cooling
    }
}
