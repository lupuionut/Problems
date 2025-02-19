// 1415. The k-th Lexicographical String of All Happy Strings of Length n
// ----------------------------------------------------------------------

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        let mut results = vec![];
        fn make(curr: &mut Vec<char>, n: i32, results: &mut Vec<String>) {
            let l = curr.len();
            let options = ['a', 'b', 'c'];
            if l == n as usize {
                results.push(curr.iter().collect::<String>());
                return;
            }
            let last = if l >= 1 { curr[l - 1] } else { 'd' };
            for &option in &options {
                if option != last {
                    curr.push(option);
                    make(curr, n, results);
                    curr.pop();
                }
            }
        }

        make(&mut vec![], n, &mut results);
        results.sort();

        if k as usize > results.len() {
            return "".to_string();
        } else {
            return results[k as usize - 1].clone();
        }
    }
}
