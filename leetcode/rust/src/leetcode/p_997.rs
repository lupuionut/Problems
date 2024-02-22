// 997. Find the Town Judge
// ------------------------

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut count = vec![(0, 0); (n + 1) as usize];
        trust.iter().for_each(|p| {
            count[p[0] as usize].0 += 1;
            count[p[1] as usize].1 += 1;
        });

        for i in 1..=n as usize {
            if count[i].0 == 0 && count[i].1 == (n - 1) {
                return i as i32;
            }
        }

        -1
    }
}
