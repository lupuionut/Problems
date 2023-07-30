// 973. K Closest Points to Origin
// -------------------------------

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut distances: Vec<(&Vec<i32>, i32)> = vec![];
        let mut ans: Vec<Vec<i32>> = vec![];

        points.iter().for_each(|p| {
            let distance = p[0] * p[0] + p[1] * p[1];
            distances.push((p, distance));
        });

        distances.sort_by(|a, b| a.1.cmp(&b.1));

        let mut ds = distances.iter().take(k as usize);
        while let Some((v, _)) = ds.next() {
            ans.push(v.to_vec());
        }
        ans
    }
}
