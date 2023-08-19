// 1023. Camelcase Matching
// ------------------------

impl Solution {
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut ans = vec![];
        queries.iter().for_each(|q| {
            let res = does_match(&q.as_bytes(), &pattern.as_bytes());
            ans.push(res);
        });

        fn does_match(q: &[u8], p: &[u8]) -> bool {
            if q.len() == 0 && p.len() == 0 {
                return true;
            }
            if q.len() == 0 {
                return false;
            }
            if p.len() == 0 {
                for i in 0..q.len() {
                    if q[i].is_ascii_uppercase() {
                        return false;
                    }
                }
                return true;
            }
            if q[0] == p[0] {
                return does_match(&q[1..], &p[1..]);
            } else {
                if !q[0].is_ascii_uppercase() {
                    return does_match(&q[1..], &p);
                } else {
                    return false;
                }
            }
        }

        ans
    }
}
