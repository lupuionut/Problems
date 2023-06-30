// 2451. Odd String Difference
// ---------------------------

impl Solution {
    pub fn odd_string(words: Vec<String>) -> String {
        let mut grouped: Vec<Vec<usize>> = vec![vec![]; 2];
        let mut delta = vec![0; words[0].len() - 1];
        let a = words[0].as_bytes();

        for i in 1..a.len() {
            delta[i - 1] = a[i] - a[i - 1];
        }

        grouped[0].push(0);

        'mainloop: for i in 1..words.len() {
            if (grouped[0].len() >= 1 && grouped[1].len() > 1)
                || (grouped[0].len() > 1 && grouped[1].len() >= 1)
            {
                break;
            }

            let a = words[i].as_bytes();
            for j in 1..a.len() {
                if delta[j - 1] != (a[j] - a[j - 1]) {
                    grouped[1].push(i);
                    continue 'mainloop;
                }
            }
            grouped[0].push(i);
        }

        if grouped[0].len() == 1 {
            words[grouped[0][0]].clone()
        } else {
            words[grouped[1][0]].clone()
        }
    }
}
