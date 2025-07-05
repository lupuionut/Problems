// 1394. Find Lucky Integer in an Array
// ------------------------------------

impl Solution {
    pub fn find_lucky(mut arr: Vec<i32>) -> i32 {
        arr.sort();
        let mut streak = 0;
        let mut last = 0;
        let mut ans = -1;
        for i in 0..arr.len() {
            if arr[i] == last {
                streak += 1;
            } else {
                if streak == last && streak > 0 {
                    ans = last;
                }
                streak = 1;
                last = arr[i];
            }
        }
        if streak == last && streak > 0 {
            ans = last;
        }
        ans
    }
}
