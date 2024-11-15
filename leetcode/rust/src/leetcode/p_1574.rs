// 1574. Shortest Subarray to be Removed to Make Array Sorted
// ----------------------------------------------------------

impl Solution {
    pub fn find_length_of_shortest_subarray(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut ans = 0;
        let mut left = 0;
        let mut right = n - 1;

        while right > 0 && arr[right] >= arr[right - 1] {
            right -= 1;
        }
        ans = right;

        while left < right {
            while right < n && arr[left] > arr[right] {
                right += 1;
            }
            ans = ans.min(right - left - 1);
            if arr[left] > arr[left + 1] {
                break;
            }
            left += 1;
        }

        ans as i32
    }
}
