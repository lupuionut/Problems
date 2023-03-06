package leetcode

/*
Input: arr = [2,3,4,7,11], k = 5
Output: 9
Explanation: The missing positive integers are [1,5,6,8,9,10,12,13,...].
The 5th missing positive integer is 9.
*/

func FindKthPositive(arr []int, k int) int {

    if arr[0] != 1 {
        arr = append([]int{1}, arr...)
        if k == 1 {
            return 1
        }
        k -= 1
    }

    for i := 0; i < len(arr) - 1; i++ {
        delta := arr[i+1] - arr[i]
        if delta > 1 {
            for j := 1; j < delta; j++ {
                k -= 1
                if k == 0 {
                    return arr[i] + j
                }
            }
        }
    }
    return arr[len(arr) - 1] + k
}

