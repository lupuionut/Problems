// 1013. Partition Array Into Three Parts With Equal Sum
// -----------------------------------------------------

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let n = arr.len() + 1;
        let mut prefix_sum = vec![0; n];
        for i in 1..n {
            prefix_sum[i] = prefix_sum[i - 1] + arr[i - 1];
        }
        let sum = prefix_sum.last().unwrap();

        if sum % 3 != 0 {
            return false;
        }

        let target = sum / 3;
        let mut first = false;

        for i in 1..n - 1 {
            if prefix_sum[i] == target && first == false {
                first = true;
                continue;
            }
            if prefix_sum[i] == target * 2 && first == true {
                return true;
            }
        }

        false
    }
}
