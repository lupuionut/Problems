// 1335. Minimum Difficulty of a Job Schedule
// ------------------------------------------

impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        fn dp(i: usize, day: i32, end: i32, jd: &Vec<i32>, cache: &mut Vec<Vec<i32>>) -> i32 {
            if i == jd.len() {
                if day == end {
                    return 0;
                }
                return i32::MAX;
            }

            if day == end {
                return i32::MAX;
            }

            if cache[i][day as usize] != -1 {
                return cache[i][day as usize];
            }

            let mut best = i32::MAX;
            let mut biggest = 0;
            for j in i..jd.len() {
                biggest = biggest.max(jd[j]);
                best = best.min(add(biggest, dp(j + 1, day + 1, end, jd, cache)));
            }
            cache[i][day as usize] = best;
            best
        }

        fn add(a: i32, b: i32) -> i32 {
            if b == i32::MAX {
                b
            } else {
                a + b
            }
        }

        let mut cache = vec![vec![-1; (d + 1) as usize]; job_difficulty.len()];
        let ans = dp(0, 0, d, &job_difficulty, &mut cache);
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
