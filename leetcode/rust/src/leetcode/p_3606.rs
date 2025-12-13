// 3606. Coupon Code Validator
// ---------------------------
impl Solution {
    pub fn validate_coupons(
        code: Vec<String>,
        business_line: Vec<String>,
        is_active: Vec<bool>,
    ) -> Vec<String> {
        let mut categories: Vec<Vec<String>> = vec![vec![]; 4];
        for i in 0..code.len() {
            if is_active[i]
                && code[i].len() > 0
                && code[i].chars().all(|c| c.is_alphanumeric() || c == '_')
            {
                if business_line[i] == "electronics".to_string() {
                    categories[0].push(code[i].clone());
                } else if business_line[i] == "grocery".to_string() {
                    categories[1].push(code[i].clone());
                } else if business_line[i] == "pharmacy".to_string() {
                    categories[2].push(code[i].clone());
                } else if business_line[i] == "restaurant".to_string() {
                    categories[3].push(code[i].clone());
                }
            }
        }
        categories
            .into_iter()
            .flat_map(|mut cat| {
                cat.sort();
                cat
            })
            .collect()
    }
}
