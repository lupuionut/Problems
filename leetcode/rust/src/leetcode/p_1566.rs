// 1566. Detect Pattern of Length M Repeated K or More Times
// ---------------------------------------------------------

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        if m * k > arr.len() as i32 {
            return false;
        }
        let m = m as usize;
        for i in 0..arr.len() - m {
            let current = &arr[i..i + m];
            let mut repetition = 1;
            let mut j = i + m;
            while j <= arr.len() - m {
                if &arr[j..j + m] == current {
                    repetition += 1;
                    j += m;
                    if repetition == k {
                        return true;
                    }
                } else {
                    break;
                }
            }
        }

        false
    }
}
