// 852. Peak Index in a Mountain Array

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = arr.len();

        while left <= right {
            let middle = (left + right) / 2;
            if arr[middle] > arr[middle - 1] && arr[middle] > arr[middle + 1] {
                return middle as i32;
            }
            if arr[middle] > arr[middle - 1] {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        return left as i32;
    }
}
