// 539. Minimum Time Difference
// ----------------------------

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        let mut times = vec![];
        fn str_to_int(s: String) -> i32 {
            let parts: Vec<&str> = s.split(':').collect();
            let mut val = 0;
            let hour = parts[0].parse::<i32>().unwrap();
            let mins = parts[1].parse::<i32>().unwrap();
            val = hour * 3600 + mins * 60;
            val
        }
        time_points.iter().for_each(|s| {
            let t = str_to_int(s.to_string());
            times.push(t);
        });
        times.sort();

        let mut ans = 24 * 3600 + times[0] - times[times.len() - 1];
        let mut iter = times.windows(2);
        while let Some(&[t0, t1]) = iter.next() {
            ans = ans.min(t1 - t0);
        }
        ans / 60
    }
}
