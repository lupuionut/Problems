// 859. Buddy Strings
// ------------------

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let mut diffs: Vec<char> = vec![];
        let mut diffg: Vec<char> = vec![];
        let mut sc = s.chars();
        let mut gc = goal.chars();
        let mut freq = vec![0; 26];
        let mut iter = sc.zip(gc);

        if s.len() != goal.len() || s.len() == 1 || goal.len() == 1 {
            return false;
        }

        while let Some((a, b)) = iter.next() {
            if a != b {
                diffs.push(a.clone());
                diffg.push(b.clone());
            }
            let k = ((a as i32) - 97) as usize;
            freq[k] += 1;
        }

        if diffs.len() == 0 {
            if *freq.iter().max().unwrap() > 1 {
                return true;
            } else {
                return false;
            }
        }

        if diffs.len() == 2 && diffg.len() == 2 && diffs[0] == diffg[1] && diffs[1] == diffg[0] {
            return true;
        }

        false
    }
}
