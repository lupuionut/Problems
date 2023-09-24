// 799. Champagne Tower
// --------------------

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        let mut current_row = vec![poured as f64];
        let mut query_row = query_row;

        while query_row > 0 {
            let mut new_row = vec![0.0; current_row.len() + 1];
            current_row.iter().enumerate().for_each(|(k, &v)| {
                if v > 1.0 {
                    let excess = (v - 1.0) / 2.0;
                    new_row[k] += excess;
                    new_row[k + 1] += excess;
                }
            });
            current_row = new_row;
            query_row -= 1;
        }

        current_row[query_glass as usize].min(1.0)
    }
}
