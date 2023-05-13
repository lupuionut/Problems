// 667. Beautiful Arrangement II

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut last = 1;
        let mut inserted = vec![0; (n + 1) as usize];
        let mut res = Vec::new();

        inserted.insert(last as usize, last);
        res.push(last);

        for i in 0..k - 1 {
            let idx = k - i;
            last = last - idx;
            if last <= 0 {
                last += (2 * idx);
            } else {
                if inserted[last as usize] != 0 {
                    last += (2 * idx);
                }
            }
            inserted[last as usize] = last;
            res.push(last);
        }

        for i in 1..n + 1 {
            if inserted[i as usize] == 0 {
                res.push(i);
            }
        }
        res
    }
}
