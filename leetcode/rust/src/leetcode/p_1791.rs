// 1791. Find Center of Star Graph
// -------------------------------

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len();
        let mut indegree = vec![0; n + 2];

        for i in 0..n {
            let edge = &edges[i];
            indegree[edge[0] as usize] += 1;
            indegree[edge[1] as usize] += 1;
        }

        let center = indegree
            .iter()
            .enumerate()
            .find(|(k, &v)| v as usize == n)
            .unwrap();
        center.0 as i32
    }
}
