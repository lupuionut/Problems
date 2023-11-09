// 1759. Count Number of Homogenous Substrings
// -------------------------------------------

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut current_char = None;
        let mut current_acc: i64 = 0;
        let mut iter = s.chars();
        let mut ans = 0;

        loop {
            let c = iter.next();
            if c.is_none() {
                break;
            }

            if c == current_char {
                current_acc += 1;
            } else {
                let sum = current_acc * (current_acc + 1) / 2;
                ans += (sum % 1_000_000_007) as i32;
                current_char = c;
                current_acc = 1;
            }
        }

        let sum = current_acc * (current_acc + 1) / 2;
        ans += (sum % 1_000_000_007) as i32;

        ans % 1_000_000_007
    }
}
