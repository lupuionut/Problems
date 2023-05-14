package leetcode

/*
    1799. Maximize Score After N Operations
    ---------------------------------------
*/

func maxScore(nums []int) int {

    cache := make([]int, 1 << len(nums))
    for k := range cache {
        cache[k] = -1
    }

    gcd := func(a, b int) int {
        for a != b {
            if a > b {
                a -= b
            } else {
                b -= a
            }
        }
        return a
    }

    max := func(a, b int) int {
        if a > b {
            return a
        }
        return b
    }

    var dfs func(mask, p int) int
    dfs = func(mask, p int) int {
        if cache[mask] != -1 {
            return cache[mask]
        }
        best := 0
        for i := 0; i < len(nums); i++ {
            if mask & (1 << i) == 0 {
                for j := i+1; j < len(nums); j++ {
                    if mask & (1 << j) == 0 {
                        score := p * gcd(nums[i], nums[j])
                        nmask := mask | (1 << i) | (1 << j)
                        best = max(best, score + dfs(nmask, p+1))
                    }
                }
            }
        }
        cache[mask] = best
        return best
    }

    return dfs(0, 1)
}
