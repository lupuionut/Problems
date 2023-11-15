// 1846. Maximum Element After Decreasing and Rearranging
// ------------------------------------------------------

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        let mut arr = arr;
        arr.sort();
        let mut curr = 0;

        arr.iter().for_each(|&n| {
            if n - curr > 1 {
                curr += 1;
            } else {
                curr = n;
            }
        });

        curr
    }
}
