// 1255. Maximum Score Words Formed by Letters
// -------------------------------------------

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        fn get_best(
            i: usize,
            words: &Vec<Vec<i32>>,
            letters: &mut Vec<i32>,
            score: &Vec<i32>,
        ) -> i32 {
            if i == words.len() {
                return 0;
            }
            let mut best = 0;

            // use letters for this word
            let mut possible = true;
            let mut value = 0;
            let mut available = letters.clone();
            for j in 0..26 {
                if words[i][j] > letters[j] {
                    possible = false;
                    break;
                }
                value += (words[i][j] * score[j]);
                letters[j] -= words[i][j];
            }

            if possible == false {
                *letters = available.clone();
            } else {
                best = best.max(value + get_best(i + 1, words, letters, score));
            }

            // skip this word
            *letters = available;
            best = best.max(get_best(i + 1, words, letters, score));

            best
        }

        let mut available = vec![0; 26];
        letters.iter().for_each(|&c| {
            let key = c as usize - 97;
            available[key] += 1;
        });

        let mut words = words
            .into_iter()
            .map(|s| {
                let mut v = vec![0; 26];
                s.chars().for_each(|c| {
                    let key = c as usize - 97;
                    v[key] += 1;
                });
                v
            })
            .collect::<Vec<_>>();

        get_best(0, &words, &mut available, &score)
    }
}
