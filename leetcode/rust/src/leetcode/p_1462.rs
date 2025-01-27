// 1462. Course Schedule IV
// ------------------------

use std::collections::HashSet;

impl Solution {
    pub fn check_if_prerequisite(
        num_courses: i32,
        prerequisites: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut req = vec![HashSet::new(); num_courses as usize];
        let mut done = vec![false; num_courses as usize];
        let mut before = vec![vec![]; num_courses as usize];
        let mut ans = vec![false; queries.len()];

        for i in 0..prerequisites.len() {
            let a = prerequisites[i][0];
            let b = prerequisites[i][1];
            before[b as usize].push(a);
        }

        fn dfs(
            i: i32,
            before: &Vec<Vec<i32>>,
            r: &mut Vec<HashSet<i32>>,
            done: &mut Vec<bool>,
        ) -> HashSet<i32> {
            if done[i as usize] == true {
                return r[i as usize].clone();
            }
            let mut ans = HashSet::new();
            ans.insert(i);
            for j in 0..before[i as usize].len() {
                let vals = dfs(before[i as usize][j], before, r, done);
                for v in vals {
                    ans.insert(v);
                }
            }
            r[i as usize] = ans.clone();
            done[i as usize] = true;
            ans
        }

        for i in 0..num_courses {
            let _ = dfs(i, &before, &mut req, &mut done);
        }

        for i in 0..queries.len() {
            let u = queries[i][0];
            let v = queries[i][1];
            if req[v as usize].contains(&u) {
                ans[i] = true;
            }
        }

        ans
    }
}
