// 2011. Final Value of Variable After Performing Operations
// ---------------------------------------------------------
impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut ans = 0;
        operations.into_iter().for_each(|o| match o.as_str() {
            "++X" | "X++" => ans += 1,
            "--X" | "X--" => ans -= 1,
            _ => {}
        });
        ans
    }
}
