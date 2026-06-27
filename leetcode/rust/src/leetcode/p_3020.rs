// 3020. Find the Maximum Number of Elements in Subset
// ---------------------------------------------------
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn maximum_length(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut f = HashMap::new();
        for &num in &nums {
            f.entry(num).and_modify(|v| *v+=1).or_insert(1);
        }

        let mut ans = 1;
        let mut visited = HashSet::new();
        for &num in &nums {
            let mut t = 0;
            let mut n = num;
            loop {
                if visited.contains(&n) {
                    break;
                }
                visited.insert(n);
                if let Some(&fq) = f.get(&n) {
                    t += 1;
                    if fq == 1 {
                        break;
                    } 
                    n = n * n;
                } else {
                    break;
                }
            }
            ans = ans.max(t);
        }

        // special case 1
        if let Some(fq) = f.get(&1) {
            ans = ans.max((fq + 1)/2);
        }
        if ans == 1 {
            return 1;
        }
        (2 * (ans - 1)) + 1
    }
}
