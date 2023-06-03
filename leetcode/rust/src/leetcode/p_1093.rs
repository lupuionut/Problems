// 1093. Statistics from a Large Sample
// ------------------------------------

impl Solution {
    pub fn sample_stats(count: Vec<i32>) -> Vec<f64> {
        let mut freq: Vec<(u64, u64)> = Vec::new();
        let mut ans: Vec<f64> = Vec::new();

        count.iter().enumerate().fold(&mut freq, |mut acc, (k, v)| {
            if *v != 0 {
                acc.push((k as u64, *v as u64));
            }
            acc
        });

        let min = freq[0].0;
        let max = freq[freq.len() - 1].0;
        let mut count = 0;
        let mut sum = 0;
        let mut mode: (u64, u64) = (0, 0);
        let mut median: f64 = 0.0;

        for (k, v) in &freq {
            sum += (k * v);
            count += v;
            if *v > mode.1 {
                mode.0 = *k;
                mode.1 = *v;
            }
        }

        if count % 2 == 0 {
            let mut counter = 0;
            let f = count / 2;
            let s = f + 1;
            let mut a: u64 = 0;
            let mut b: u64 = 0;

            for (k, v) in &freq {
                counter += v;
                if f <= counter && a == 0 {
                    a = *k;
                }
                if s <= counter {
                    b = *k;
                    break;
                }
            }
            median = (a + b) as f64 / 2 as f64;
        } else {
            let mut counter = 0;
            let search = (count / 2) + 1;

            for (k, v) in &freq {
                counter += v;
                if search <= counter {
                    median = *k as f64;
                    break;
                }
            }
        }

        ans.push(min as f64);
        ans.push(max as f64);
        ans.push(sum as f64 / count as f64);
        ans.push(median);
        ans.push(mode.0 as f64);

        ans
    }
}
