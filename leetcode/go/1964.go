package leetcode

/*
    1964. Find the Longest Valid Obstacle Course at Each Position
    -------------------------------------------------------------
*/

func longestObstacleCourseAtEachPosition(obstacles []int) []int {
    ans := []int{}
    dp := []int{}

    binarySearch := func(xs []int, target int) int {
        l := 0
        r := len(xs) - 1

        for l <= r {
            middle := (l + r) / 2
            if xs[middle] <= target {
                l = middle + 1
            } else {
                r = middle - 1
            }
        }
        return l
    }

    for k := range obstacles {
        index := binarySearch(dp, obstacles[k])
        if index >= len(dp) {
            dp = append(dp, obstacles[k])
        } else {
            dp[index] = obstacles[k]
        }
        ans = append(ans, index + 1)
    }

    return ans
}
