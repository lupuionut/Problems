// 785. Is Graph Bipartite?

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut colours = vec![0; graph.len()];

        fn bfs(i: usize, colours: &mut Vec<i32>, graph: &Vec<Vec<i32>>) -> bool {
            if colours[i] != 0 {
                return true;
            }
            colours[i] = -1;
            let mut queue = vec![];
            queue.push(i);

            while queue.len() > 0 {
                let node = queue.pop().unwrap();
                for v in &graph[node] {
                    let key = *v as usize;
                    if colours[key] == colours[node] {
                        return false;
                    }
                    if colours[key] == 0 {
                        colours[key] = colours[node] * -1;
                        queue.push(key);
                    }
                }
            }

            true
        }

        for i in 0..graph.len() {
            if bfs(i, &mut colours, &graph) == false {
                return false;
            }
        }

        true
    }
}
