// 1441. Build an Array With Stack Operations
// ------------------------------------------

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut ans: Vec<String> = vec![];
        let mut i = 1;
        let mut j = 0;

        while i <= n && j < target.len() {
            if target[j] == i {
                ans.push("Push".to_string());
                j += 1;
            } else {
                ans.push("Push".to_string());
                ans.push("Pop".to_string());
            }

            i += 1;
        }

        ans
    }
}
