// 1773. Count Items Matching a Rule
// ---------------------------------

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let idx: usize = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 3,
        };

        items
            .iter()
            .filter(|&item| item[idx] == rule_value)
            .collect::<Vec<_>>()
            .len() as i32
    }
}
