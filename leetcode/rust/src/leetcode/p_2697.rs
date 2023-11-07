// 2697. Lexicographically Smallest Palindrome
// -------------------------------------------

impl Solution {
    pub fn make_smallest_palindrome(s: String) -> String {
        let n = s.len();

        if n == 1 {
            return s;
        }

        let mut l = 0;
        let mut r = n - 1;
        let mut ans = vec!['a'; n];
        let mut cs: Vec<char> = s.chars().collect();

        while l <= r {
            if cs[l] == cs[r] {
                ans[l] = cs[l];
                ans[r] = cs[l];
            } else {
                let smallest = cs[l].min(cs[r]);
                ans[l] = smallest;
                ans[r] = smallest;
            }
            l += 1;
            r -= 1;
        }

        ans.iter().collect::<String>()
    }
}
