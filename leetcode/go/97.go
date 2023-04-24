package leetcode

/*
    97. Interleaving String
    -----------------------
    Traverse s3 by keeping track of indexes for each string: s1,s2,s3
    Each time we check for a char, we have options:
    - equal for s1 s2 and s3, in this case we must recurse into 2 other cases
    - equal for s1 and s3 or s2 and s3 and we continue to next char
    - neither one equal so it's false
*/

func isInterleave(s1 string, s2 string, s3 string) bool {
    n := len(s1)
    m := len(s2)
    cache := make([][]int, n)
    for i := range cache {
        cache[i] = make([]int, m)
        for j := range cache[i] {
            cache[i][j] = -1
        }
    }

    if m + n != len(s3) {
        return false
    }

    var dfs func(i, j, k int) bool
    dfs = func(i, j, k int) bool {
        if i == n {
            return s2[j:] == s3[k:]
        }
        if j == m {
            return s1[i:] == s3[k:]
        }

        if cache[i][j] != -1 {
            if cache[i][j] == 1 {
                return true
            }
            return false
        }

        if s3[k] == s1[i] && s3[k] == s2[j] {
            res := dfs(i+1, j, k+1) || dfs(i, j+1, k+1)
            if res == true {
                cache[i][j] = 1
            } else {
                cache[i][j] = 0
            }
            return res
        }
        if s3[k] == s1[i] {
            res := dfs(i+1, j, k+1)
            if res == true {
                cache[i][j] = 1
            } else {
                cache[i][j] = 0
            }
            return res
        }
        if s3[k] == s2[j] {
            res := dfs(i, j+1, k+1)
            if res == true {
                cache[i][j] = 1
            } else {
                cache[i][j] = 0
            }
            return res
        }
        return false
    }

    return dfs(0,0,0)
}
