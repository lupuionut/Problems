// 2698. Find the Punishment Number of an Integer
// ----------------------------------------------

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        fn can_split(n: i32) -> bool {
            let mut digits = vec![];
            let mut square = n * n;
            while square > 0 {
                digits.push(square % 10);
                square /= 10;
            }
            digits = digits.into_iter().rev().collect();

            fn dfs(i: usize, curr: i32, target: i32, digits: &Vec<i32>) -> bool {
                if i == digits.len() {
                    return curr == target;
                }
                let s = curr * 10 + digits[i];
                dfs(i + 1, 0, target - s, digits) || dfs(i + 1, s, target, digits)
            }
            dfs(0, 0, n, &digits)
        }
        let mut ans = 0;
        for i in 1..=n {
            if can_split(i) {
                ans += i * i;
            }
        }
        ans
    }
}
