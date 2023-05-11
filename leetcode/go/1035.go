package leetcode

/*
    1035. Uncrossed Lines
    ---------------------
*/

func maxUncrossedLines(nums1 []int, nums2 []int) int {
    cache := make([][]int, len(nums1))
    for k := range cache {
        cache[k] = make([]int, len(nums2))
        for j := range cache[k] {
            cache[k][j] = -1
        }
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var dfs func(i, j int) int
    dfs = func(i, j int) int {

        if len(nums1) == i || len(nums2) == j {
            return 0
        }
        if cache[i][j] != -1 {
            return cache[i][j]
        }

        ans := 0
        if nums1[i] == nums2[j] {
            ans = max(ans, 1 + dfs(i+1, j+1))
        } else {
            ans = max(ans, max(dfs(i+1, j), dfs(i, j+1)))
        }
        cache[i][j] = ans
        return cache[i][j]
    }

    ans := dfs(0,0)
    return ans
}
