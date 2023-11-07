// 1921. Eliminate Maximum Number of Monsters
// ------------------------------------------

impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut arrivals = vec![0; 100_001];
        let mut iter = dist.iter().zip(speed.iter());

        while let Some((&d, &s)) = iter.next() {
            let mut time = d / s;
            if d % s != 0 {
                time += 1;
            }
            arrivals[time as usize] += 1;
        }

        let mut ans = 0;
        let mut gun_ready = 0;

        for (k, &v) in arrivals.iter().enumerate() {
            if v == 0 {
                continue;
            }
            let time = k as i32;
            if gun_ready + v > time {
                ans += (time - gun_ready);
                return ans;
            } else {
                ans += v;
                gun_ready += v;
            }
        }

        ans
    }
}
