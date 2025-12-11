// 3531. Count Covered Buildings
// -----------------------------
use std::collections::HashMap;
impl Solution {
    pub fn count_covered_buildings(n: i32, mut buildings: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut ox: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut oy: HashMap<i32, Vec<i32>> = HashMap::new();

        buildings.sort();

        for b in &buildings {
            let x = b[0];
            let y = b[1];
            ox.entry(x).and_modify(|v| v.push(y)).or_insert(vec![y]);
            oy.entry(y).and_modify(|v| v.push(x)).or_insert(vec![x]);
        }

        for b in &buildings {
            let x = b[0];
            let y = b[1];
            if let Some(ys) = ox.get(&x) {
                if let Some(xs) = oy.get(&y) {
                    if ys[0] != y && ys.last() != Some(&y) && xs[0] != x && xs.last() != Some(&x) {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}
