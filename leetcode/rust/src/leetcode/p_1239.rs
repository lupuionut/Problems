// 1239. Maximum Length of a Concatenated String with Unique Characters
// --------------------------------------------------------------------

impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        fn rec(
            i: usize,
            arr: &Vec<String>,
            used_pos: i32,
            used_chars: &mut [i32],
            cache: &mut Vec<Vec<i32>>,
        ) -> i32 {
            if i == arr.len() {
                return 0;
            }
            if cache[i][used_pos as usize] != -1 {
                return cache[i][used_pos as usize];
            }

            let mut can_use = true;
            // mark each visited char as used, so we can check for duplicates in curr string
            arr[i].chars().for_each(|c| {
                let key = (c as usize) - 97;
                used_chars[key] += 1;
                if used_chars[key] > 1 {
                    can_use = false;
                }
            });

            let mut best = 0;
            if can_use {
                let used_pos = (1 << (i as i32)) | used_pos;
                best = arr[i].len() as i32 + best.max(rec(i + 1, arr, used_pos, used_chars, cache));
            }

            // reverse used chars
            arr[i].chars().for_each(|c| {
                let key = (c as usize) - 97;
                used_chars[key] -= 1;
            });

            best = best.max(rec(i + 1, arr, used_pos, used_chars, cache));
            cache[i][used_pos as usize] = best;
            cache[i][used_pos as usize]
        }

        let mut used_chars = [0; 26];
        let max = 1 << 16 as usize;
        let mut cache = vec![vec![-1; max]; arr.len()];
        rec(0, &arr, 0, &mut used_chars, &mut cache)
    }
}
