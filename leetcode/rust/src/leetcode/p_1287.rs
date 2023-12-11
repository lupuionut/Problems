// 1287. Element Appearing More Than 25% In Sorted Array
// -----------------------------------------------------

impl Solution {
    pub fn find_special_integer(arr: Vec<i32>) -> i32 {
        let mut max_freq = 0;
        let mut max_numb = 0;
        let mut curr_numb = -1;
        let mut curr_freq = 0;

        arr.iter().for_each(|&n| {
            if n != curr_numb {
                curr_numb = n;
                curr_freq = 1;
            } else {
                curr_freq += 1;
            }
            if curr_freq > max_freq {
                max_freq = curr_freq;
                max_numb = n;
            }
        });

        max_numb
    }
}
