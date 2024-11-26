// 2924. Find Champion II
// ----------------------

impl Solution {
    pub fn find_champion(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut inbound = vec![0; n as usize];
        for i in 0..edges.len() {
            inbound[edges[i][1] as usize] += 1;
        }
        let winners = inbound
            .iter()
            .enumerate()
            .filter(|(k, &v)| v == 0)
            .map(|(k, _)| k as i32)
            .collect::<Vec<i32>>();
        if winners.len() > 1 {
            -1
        } else {
            winners[0]
        }
    }
}
