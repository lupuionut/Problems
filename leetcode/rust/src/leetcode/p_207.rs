// 207. Course Schedule
// --------------------

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut before: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];
        let mut cache: Vec<i32> = vec![-1; num_courses as usize];
        let mut visited = vec![false; num_courses as usize];

        fn dfs(
            i: usize,
            visited: &mut Vec<bool>,
            before: &Vec<Vec<i32>>,
            cache: &mut Vec<i32>,
        ) -> bool {
            let mut res = true;

            if cache[i] != -1 {
                if cache[i] == 1 {
                    return true;
                }
                return false;
            }

            for course in &before[i] {
                if visited[*course as usize] == true {
                    return false;
                } else {
                    let k = *course as usize;
                    visited[k] = true;
                    res = res && dfs(*course as usize, visited, before, cache);
                    visited[k] = false;
                }
            }
            if res == true {
                cache[i] = 1;
            }

            res
        }

        for i in 0..prerequisites.len() {
            let p = &prerequisites[i];
            before[p[0] as usize].push(p[1]);
        }

        for i in 0..num_courses as usize {
            visited[i] = true;
            if dfs(i, &mut visited, &before, &mut cache) == false {
                return false;
            }
            visited[i] = false;
        }

        true
    }
}
