// 168. Excel Sheet Column Title
// -----------------------------

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut ans = vec![];
        let mut d = column_number;

        while d > 0 {
            d -= 1;
            ans.push(char::from_u32(65 + d as u32 % 26).unwrap());
            d /= 26;
        }

        ans.into_iter().rev().collect::<String>()
    }
}
