// 1557. Minimum Number of Vertices to Reach All Nodes

impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut incoming = vec![false; n as usize];
        let mut ans: Vec<i32> = Vec::new();

        for edge in edges {
            let key = edge[1];
            incoming[key as usize] = true;
        }
        for (k, v) in incoming.iter().enumerate() {
            if *v == false {
                ans.push(k as i32);
            }
        }

        ans
    }
}
