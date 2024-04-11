// 402. Remove K Digits
// --------------------

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = vec![];
        let mut num = num.chars().collect::<Vec<char>>();
        let mut k = k;

        for i in 0..num.len() {
            while stack.len() > 0 && k > 0 && *stack.last().unwrap() > num[i] {
                stack.pop();
                k -= 1;
            }
            stack.push(num[i]);
        }

        while k > 0 && stack.len() > 0 {
            stack.pop();
            k -= 1;
        }

        let stack = stack.iter().skip_while(|&c| *c == '0').collect::<Vec<_>>();
        let ans = if stack.len() > 0 {
            stack.into_iter().collect::<String>()
        } else {
            "0".to_string()
        };

        ans
    }
}
