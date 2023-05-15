// 306. Additive Number

impl Solution {
    pub fn is_additive_number(num: String) -> bool {
        let n = num.len();
        for i in 0..n {
            for j in 0..i {
                let num1 = num.get(0..j + 1).unwrap();
                let num2 = num.get(j + 1..i + 1).unwrap();
                let num3 = num.get(i + 1..n).unwrap();
                if Solution::dfs(&num1, &num2, &num3) == true {
                    return true;
                }
            }
        }
        false
    }

    pub fn dfs(num1: &str, num2: &str, num3: &str) -> bool {
        if num1.len() == 0 || num2.len() == 0 || num3.len() == 0 {
            return false;
        }
        if (num1.len() > 1 && num1.starts_with('0'))
            || (num2.len() > 1 && num2.starts_with('0'))
            || (num3.len() > 1 && num3.starts_with('0'))
        {
            return false;
        }
        let a = num1.parse::<i64>().unwrap();
        let b = num2.parse::<i64>().unwrap();
        let c = (a + b).to_string();

        if c.len() == num3.len() {
            if c == num3 {
                return true;
            }
        }
        if c.len() > num3.len() {
            return false;
        }
        if num3.starts_with(&c) {
            let remains = num3.get(c.len()..num3.len()).unwrap();
            return Solution::dfs(num2, &c, &remains);
        }
        false
    }
}
