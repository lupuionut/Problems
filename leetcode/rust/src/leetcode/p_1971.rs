// 1971. Find if Path Exists in Graph
// ----------------------------------

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut conn = vec![vec![]; n as usize];
        let mut visited = vec![false; n as usize];
        edges.iter().for_each(|edge| {
            conn[edge[0] as usize].push(edge[1]);
            conn[edge[1] as usize].push(edge[0]);
        });
        let mut queue = vec![source];
        while queue.len() > 0 {
            let curr = queue.pop().unwrap();
            if curr == destination {
                return true;
            }
            visited[curr as usize] = true;
            let next = &conn[curr as usize];
            for i in 0..next.len() {
                if visited[next[i] as usize] == false {
                    queue.push(next[i]);
                }
            }
        }

        false
    }
}
