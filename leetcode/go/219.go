package leetcode

/*
    219. Contains Duplicate II
    --------------------------
    If k == 0, then no duplicates is possible since the diff
    between 2 indices is at least 1.
    Otherwise use a map and for each number keep the last index
    so that we can compare it with the current index. If the
    difference is bigger than k, keep the current index,
    otherwise we have a smaller difference and we return true.
*/

func containsNearbyDuplicate(nums []int, k int) bool {

    if k == 0 {
        return false
    }

    previous := map[int]int{}
    for i := range nums {
        if idx, ok := previous[nums[i]]; ok {
            if i - idx > k {
                previous[nums[i]] = i
            } else {
                return true
            }
        } else {
            previous[nums[i]] = i
        }
    }
    return false
}
