// 67. Add Binary
// --------------
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        fn add(a: char, b: char, c: char) -> (char, char) {
            let mut res = ('0', '0');
            match (a, b, c) {
                ('0', '0', '1') => res = ('1', '0'),
                ('0', '1', '0') => res = ('1', '0'),
                ('0', '1', '1') => res = ('0', '1'),
                ('1', '0', '0') => res = ('1', '0'),
                ('1', '0', '1') => res = ('0', '1'),
                ('1', '1', '0') => res = ('0', '1'),
                ('1', '1', '1') => res = ('1', '1'),
                _ => {}
            }
            res
        }
        let mut ch = vec![];
        let mut carry = '0';
        let a = a.chars().rev().collect::<Vec<char>>();
        let b = b.chars().rev().collect::<Vec<char>>();
        let m = a.len();
        let n = b.len();
        let j = m.max(n);
        for i in 0..j {
            let mut res = ('0', '0');
            if i >= m {
                res = add('0', b[i], carry);
            } else if i >= n {
                res = add(a[i], '0', carry);
            } else {
                res = add(a[i], b[i], carry);
            }
            carry = res.1;
            ch.push(res.0);
        }
        if carry == '1' {
            ch.push('1');
        }
        ch.iter().rev().collect::<String>()
    }
}
