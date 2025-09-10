// 1733. Minimum Number of People to Teach
// ---------------------------------------

use std::collections::HashSet;
impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut ans = i32::MAX;

        let mut friends = vec![];
        for i in 0..languages.len() {
            let mut f = HashSet::new();
            for j in 0..languages[i].len() {
                f.insert(languages[i][j]);
            }
            friends.push(f);
        }

        let mut issues = vec![];
        for f in friendships {
            let mut understand = false;
            let l0 = &friends[(f[0] - 1) as usize];
            let l1 = &friends[(f[1] - 1) as usize];
            let common = l0.intersection(&l1);
            if common.count() == 0 {
                issues.push((f[0], f[1]));
            }
        }

        for i in 1..=n {
            let mut t = 0;
            for &(f0,f1) in &issues {   
                if friends[(f0 - 1) as usize].insert(i) {
                    t += 1;
                }
                if friends[(f1 - 1) as usize].insert(i) {
                    t += 1;
                }
            }
            ans = ans.min(t);
        }
        ans
    }
}
